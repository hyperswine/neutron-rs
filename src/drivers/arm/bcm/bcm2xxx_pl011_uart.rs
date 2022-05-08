// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

// PL011 UART driver.
// - <https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf>
// - <https://developer.arm.com/documentation/ddi0183/latest>

use crate::{
    bsp, bsp::device_driver::common::MMIODerefWrapper, console, cpu, driver, exception, memory,
    synchronization, synchronization::IRQSafeNullLock,
};
use core::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

// Private Definitions

// PL011 UART registers.
register_bitfields! {
    u32,

    /// Flag Register.
    FR [
        TXFE OFFSET(7) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer Baud Rate Divisor.
    IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    LCR_H [
        #[allow(clippy::enum_variant_names)]
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        /// Enable FIFOs:
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    /// Control Register.
    CR [
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt FIFO Level Select Register.
    IFLS [
        RXIFLSEL OFFSET(3) NUMBITS(5) [
            OneEigth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEights = 0b100
        ]
    ],

    /// Interrupt Mask Set/Clear Register.
    IMSC [
        RTIM OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RXIM OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Masked Interrupt Status Register.
    MIS [
        RTMIS OFFSET(6) NUMBITS(1) [],
        RXMIS OFFSET(4) NUMBITS(1) []
    ],

    /// Interrupt Clear Register.
    ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => IFLS: ReadWrite<u32, IFLS::Register>),
        (0x38 => IMSC: ReadWrite<u32, IMSC::Register>),
        (0x3C => _reserved3),
        (0x40 => MIS: ReadOnly<u32, MIS::Register>),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

#[derive(PartialEq)]
enum BlockingMode {
    Blocking,
    NonBlocking,
}

//--------------------------------------------------------------------------------------------------
// Public Definitions
//--------------------------------------------------------------------------------------------------

pub struct PL011UartInner {
    registers: Registers,
    chars_written: usize,
    chars_read: usize,
}

// Export the inner struct so that BSPs can use it for the panic handler.
pub use PL011UartInner as PanicUart;

/// Representation of the UART.
pub struct PL011Uart {
    mmio_descriptor: memory::mmu::MMIODescriptor,
    virt_mmio_start_addr: AtomicUsize,
    inner: IRQSafeNullLock<PL011UartInner>,
    irq_number: bsp::device_driver::IRQNumber,
}

// Public Code

impl PL011UartInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
            chars_written: 0,
            chars_read: 0,
        }
    }

    /// Set up baud rate and characteristics.
    pub unsafe fn init(&mut self, new_mmio_start_addr: Option<usize>) -> Result<(), &'static str> {
        if let Some(addr) = new_mmio_start_addr {
            self.registers = Registers::new(addr);
        }

        // Execution can arrive here while there are still characters queued in the TX FIFO and
        self.flush();

        // Turn the UART off temporarily.
        self.registers.CR.set(0);

        // Clear all pending interrupts.
        self.registers.ICR.write(ICR::ALL::CLEAR);

        // Set the baud rate, 8N1 and FIFO enabled.
        self.registers.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.registers.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));
        self.registers
            .LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // Set RX FIFO fill level at 1/8.
        self.registers.IFLS.write(IFLS::RXIFLSEL::OneEigth);

        // Enable RX IRQ + RX timeout IRQ.
        self.registers
            .IMSC
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        // Turn the UART on.
        self.registers
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Ok(())
    }

    /// Send a character.
    fn write_char(&mut self, c: char) {
        // Spin while TX FIFO full is set, waiting for an empty slot.
        while self.registers.FR.matches_all(FR::TXFF::SET) {
            cpu::nop();
        }

        // Write the character to the buffer.
        self.registers.DR.set(c as u32);

        self.chars_written += 1;
    }

    /// Block execution until the last buffered character has been physically put on the TX wire.
    fn flush(&self) {
        // Spin until the busy bit is cleared.
        while self.registers.FR.matches_all(FR::BUSY::SET) {
            cpu::nop();
        }
    }

    /// Retrieve a character.
    fn read_char_converting(&mut self, blocking_mode: BlockingMode) -> Option<char> {
        // If RX FIFO is empty,
        if self.registers.FR.matches_all(FR::RXFE::SET) {
            // immediately return in non-blocking mode.
            if blocking_mode == BlockingMode::NonBlocking {
                return None;
            }

            // Otherwise, wait until a char was received.
            while self.registers.FR.matches_all(FR::RXFE::SET) {
                cpu::nop();
            }
        }

        // Read one character.
        let mut ret = self.registers.DR.get() as u8 as char;

        // Convert carrige return to newline.
        if ret == '\r' {
            ret = '\n'
        }

        // Update statistics.
        self.chars_read += 1;

        Some(ret)
    }
}

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

impl PL011Uart {
    pub const unsafe fn new(
        mmio_descriptor: memory::mmu::MMIODescriptor,
        irq_number: bsp::device_driver::IRQNumber,
    ) -> Self {
        Self {
            mmio_descriptor,
            virt_mmio_start_addr: AtomicUsize::new(0),
            inner: IRQSafeNullLock::new(PL011UartInner::new(
                mmio_descriptor.start_addr().as_usize(),
            )),
            irq_number,
        }
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------
use synchronization::interface::Mutex;

impl driver::interface::DeviceDriver for PL011Uart {
    fn compatible(&self) -> &'static str {
        "BCM PL011 UART"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let virt_addr = memory::mmu::kernel_map_mmio(self.compatible(), &self.mmio_descriptor)?;

        self.inner
            .lock(|inner| inner.init(Some(virt_addr.as_usize())))?;

        self.virt_mmio_start_addr
            .store(virt_addr.as_usize(), Ordering::Relaxed);

        Ok(())
    }

    fn register_and_enable_irq_handler(&'static self) -> Result<(), &'static str> {
        use bsp::exception::asynchronous::irq_manager;
        use exception::asynchronous::{interface::IRQManager, IRQDescriptor};

        let descriptor = IRQDescriptor {
            name: "BCM PL011 UART",
            handler: self,
        };

        irq_manager().register_handler(self.irq_number, descriptor)?;
        irq_manager().enable(self.irq_number);

        Ok(())
    }

    fn virt_mmio_start_addr(&self) -> Option<usize> {
        let addr = self.virt_mmio_start_addr.load(Ordering::Relaxed);

        if addr == 0 {
            return None;
        }

        Some(addr)
    }
}

impl console::interface::Write for PL011Uart {
    fn write_char(&self, c: char) {
        self.inner.lock(|inner| inner.write_char(c));
    }

    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        self.inner.lock(|inner| inner.flush());
    }
}

impl console::interface::Read for PL011Uart {
    fn read_char(&self) -> char {
        self.inner
            .lock(|inner| inner.read_char_converting(BlockingMode::Blocking).unwrap())
    }

    fn clear_rx(&self) {
        while self
            .inner
            .lock(|inner| inner.read_char_converting(BlockingMode::NonBlocking))
            .is_some()
        {}
    }
}

impl console::interface::Statistics for PL011Uart {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }

    fn chars_read(&self) -> usize {
        self.inner.lock(|inner| inner.chars_read)
    }
}

impl exception::asynchronous::interface::IRQHandler for PL011Uart {
    fn handle(&self) -> Result<(), &'static str> {
        self.inner.lock(|inner| {
            let pending = inner.registers.MIS.extract();

            // Clear all pending IRQs.
            inner.registers.ICR.write(ICR::ALL::CLEAR);

            // Check for any kind of RX interrupt.
            if pending.matches_any(MIS::RXMIS::SET + MIS::RTMIS::SET) {
                // Echo any received characters.
                while let Some(c) = inner.read_char_converting(BlockingMode::NonBlocking) {
                    inner.write_char(c)
                }
            }
        });

        Ok(())
    }
}
