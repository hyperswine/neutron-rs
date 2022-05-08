// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

//! GPIO Driver.

use crate::{
    bsp::device_driver::common::MMIODerefWrapper, driver, memory, synchronization,
    synchronization::IRQSafeNullLock,
};
use core::sync::atomic::{AtomicUsize, Ordering};
use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

// Private Definitions

// GPIO registers.
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART RX

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART TX
        ]
    ],

    /// GPIO Pull-up/down Register
    GPPUD [
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    /// BCM2837 only.
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],

    /// GPIO Pull-up / Pull-down Register 0
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => GPPUD: ReadWrite<u32, GPPUD::Register>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => _reserved3),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG0: ReadWrite<u32, GPIO_PUP_PDN_CNTRL_REG0::Register>),
        (0xE8 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;

// Public Definitions

pub struct GPIOInner {
    registers: Registers,
}

// Export the inner struct so that BSPs can use it for the panic handler.
pub use GPIOInner as PanicGPIO;

/// Representation of the GPIO HW.
pub struct GPIO {
    mmio_descriptor: memory::mmu::MMIODescriptor,
    virt_mmio_start_addr: AtomicUsize,
    inner: IRQSafeNullLock<GPIOInner>,
}

// Public Code

impl GPIOInner {
    /// Create an instance.
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: Registers::new(mmio_start_addr),
        }
    }

    /// Init code.
    pub unsafe fn init(&mut self, new_mmio_start_addr: Option<usize>) -> Result<(), &'static str> {
        if let Some(addr) = new_mmio_start_addr {
            self.registers = Registers::new(addr);
        }

        Ok(())
    }

    /// Disable pull-up/down on pins 14 and 15.
    // * For PI3
    fn disable_pud_14_15_bcm2837(&mut self) {
        use crate::{time, time::interface::TimeManager};
        use core::time::Duration;

        // The Linux 2837 GPIO driver waits 1 µs between the steps.
        const DELAY: Duration = Duration::from_micros(1);

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        time::time_manager().spin_for(DELAY);

        self.registers
            .GPPUDCLK0
            .write(GPPUDCLK0::PUDCLK15::AssertClock + GPPUDCLK0::PUDCLK14::AssertClock);
        time::time_manager().spin_for(DELAY);

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        self.registers.GPPUDCLK0.set(0);
    }

    /// Disable pull-up/down on pins 14 and 15.
    // * For PI4
    #[cfg(feature = "bsp_rpi4")]
    fn disable_pud_14_15_bcm2711(&mut self) {
        self.registers.GPIO_PUP_PDN_CNTRL_REG0.write(
            GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL15::PullUp
                + GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL14::PullUp,
        );
    }

    /// Map PL011 UART as standard output.
    /// TX to pin 14
    /// RX to pin 15
    pub fn map_pl011_uart(&mut self) {
        // Select the UART on pins 14 and 15.
        self.registers
            .GPFSEL1
            .modify(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        // Disable pull-up/down on pins 14 and 15.
        // TODO: check device == pi3. Ensure this works

        enum PiDevice {
            Pi3,
            Pi4,
        }
        let dev_ = PiDevice::Pi3;

        if dev_ == PiDevice::Pi3 {
            self.disable_pud_14_15_bcm2837();
        }
        if dev_ == PiDevice::Pi4 {
            self.disable_pud_14_15_bcm2711();
        }
    }
}

impl GPIO {
    /// Create an instance.
    pub const unsafe fn new(mmio_descriptor: memory::mmu::MMIODescriptor) -> Self {
        Self {
            mmio_descriptor,
            virt_mmio_start_addr: AtomicUsize::new(0),
            inner: IRQSafeNullLock::new(GPIOInner::new(mmio_descriptor.start_addr().as_usize())),
        }
    }

    /// Concurrency safe version of `GPIOInner.map_pl011_uart()`
    pub fn map_pl011_uart(&self) {
        self.inner.lock(|inner| inner.map_pl011_uart())
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------
use synchronization::interface::Mutex;

impl driver::interface::DeviceDriver for GPIO {
    fn compatible(&self) -> &'static str {
        "BCM GPIO"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let virt_addr = memory::mmu::kernel_map_mmio(self.compatible(), &self.mmio_descriptor)?;

        self.inner
            .lock(|inner| inner.init(Some(virt_addr.as_usize())))?;

        self.virt_mmio_start_addr
            .store(virt_addr.as_usize(), Ordering::Relaxed);

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
