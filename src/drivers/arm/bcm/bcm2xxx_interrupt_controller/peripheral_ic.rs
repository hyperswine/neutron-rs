// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2020-2022 Andre Richter <andre.o.richter@gmail.com>

// Peripheral Interrupt Controller Driver.

use super::{InterruptController, PendingIRQs, PeripheralIRQ};
use crate::{
    bsp::device_driver::common::MMIODerefWrapper,
    driver, exception, memory, synchronization,
    synchronization::{IRQSafeNullLock, InitStateLock},
};
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, WriteOnly},
};

// Private Definitions

register_structs! {
    #[allow(non_snake_case)]
    WORegisterBlock {
        (0x00 => _reserved1),
        (0x10 => ENABLE_1: WriteOnly<u32>),
        (0x14 => ENABLE_2: WriteOnly<u32>),
        (0x24 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    RORegisterBlock {
        (0x00 => _reserved1),
        (0x04 => PENDING_1: ReadOnly<u32>),
        (0x08 => PENDING_2: ReadOnly<u32>),
        (0x0c => @END),
    }
}

type WriteOnlyRegisters = MMIODerefWrapper<WORegisterBlock>;

type ReadOnlyRegisters = MMIODerefWrapper<RORegisterBlock>;

type HandlerTable =
    [Option<exception::asynchronous::IRQDescriptor>; InterruptController::NUM_PERIPHERAL_IRQS];

// Public Definitions


pub struct PeripheralIC {
    mmio_descriptor: memory::mmu::MMIODescriptor,

    wo_registers: IRQSafeNullLock<WriteOnlyRegisters>,

    ro_registers: InitStateLock<ReadOnlyRegisters>,

    handler_table: InitStateLock<HandlerTable>,
}

// Public Code

impl PeripheralIC {
    pub const unsafe fn new(mmio_descriptor: memory::mmu::MMIODescriptor) -> Self {
        let addr = mmio_descriptor.start_addr().as_usize();

        Self {
            mmio_descriptor,
            wo_registers: IRQSafeNullLock::new(WriteOnlyRegisters::new(addr)),
            ro_registers: InitStateLock::new(ReadOnlyRegisters::new(addr)),
            handler_table: InitStateLock::new([None; InterruptController::NUM_PERIPHERAL_IRQS]),
        }
    }

    
    fn pending_irqs(&self) -> PendingIRQs {
        self.ro_registers.read(|regs| {
            let pending_mask: u64 =
                (u64::from(regs.PENDING_2.get()) << 32) | u64::from(regs.PENDING_1.get());

            PendingIRQs::new(pending_mask)
        })
    }
}

//------------------------------------------------------------------------------
// OS Interface Code
//------------------------------------------------------------------------------
use synchronization::interface::{Mutex, ReadWriteEx};

impl driver::interface::DeviceDriver for PeripheralIC {
    fn compatible(&self) -> &'static str {
        "BCM Peripheral Interrupt Controller"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let virt_addr =
            memory::mmu::kernel_map_mmio(self.compatible(), &self.mmio_descriptor)?.as_usize();

        self.wo_registers
            .lock(|regs| *regs = WriteOnlyRegisters::new(virt_addr));
        self.ro_registers
            .write(|regs| *regs = ReadOnlyRegisters::new(virt_addr));

        Ok(())
    }
}

impl exception::asynchronous::interface::IRQManager for PeripheralIC {
    type IRQNumberType = PeripheralIRQ;

    fn register_handler(
        &self,
        irq: Self::IRQNumberType,
        descriptor: exception::asynchronous::IRQDescriptor,
    ) -> Result<(), &'static str> {
        self.handler_table.write(|table| {
            let irq_number = irq.get();

            if table[irq_number].is_some() {
                return Err("IRQ handler already registered");
            }

            table[irq_number] = Some(descriptor);

            Ok(())
        })
    }

    fn enable(&self, irq: Self::IRQNumberType) {
        self.wo_registers.lock(|regs| {
            let enable_reg = if irq.get() <= 31 {
                &regs.ENABLE_1
            } else {
                &regs.ENABLE_2
            };

            let enable_bit: u32 = 1 << (irq.get() % 32);

            // Writing a 1 to a bit will set the corresponding IRQ enable bit.
            enable_reg.set(enable_bit);
        });
    }

    fn handle_pending_irqs<'irq_context>(
        &'irq_context self,
        _ic: &exception::asynchronous::IRQContext<'irq_context>,
    ) {
        self.handler_table.read(|table| {
            for irq_number in self.pending_irqs() {
                match table[irq_number] {
                    None => panic!("No handler registered for IRQ {}", irq_number),
                    Some(descriptor) => {
                        descriptor.handler.handle().expect("Error handling IRQ");
                    }
                }
            }
        })
    }
}
