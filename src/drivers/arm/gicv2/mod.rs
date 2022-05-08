// GICv2 Driver - ARM Generic Interrupt Controller v2.

mod gicc;
mod gicd;

use crate::types::synchronisation::InitStateLock;
use core::sync::atomic::{AtomicBool, Ordering};

type HandlerTable = [Option<exception::asynchronous::IRQDescriptor>; GICv2::NUM_IRQS];

pub type IRQNumber = exception::asynchronous::IRQNumber<{ GICv2::MAX_IRQ_NUMBER }>;

pub struct GICv2 {
    gicd_mmio_descriptor: memory::mmu::MMIODescriptor,
    gicc_mmio_descriptor: memory::mmu::MMIODescriptor,
    gicd: gicd::GICD,
    gicc: gicc::GICC,
    is_mmio_remapped: AtomicBool,
    handler_table: InitStateLock<HandlerTable>,
}

impl GICv2 {
    // Normally 1019, but keep it lower to save some space.
    const MAX_IRQ_NUMBER: usize = 300;
    const NUM_IRQS: usize = Self::MAX_IRQ_NUMBER + 1;

    pub const unsafe fn new(
        gicd_mmio_descriptor: memory::mmu::MMIODescriptor,
        gicc_mmio_descriptor: memory::mmu::MMIODescriptor,
    ) -> Self {
        Self {
            gicd_mmio_descriptor,
            gicc_mmio_descriptor,
            gicd: gicd::GICD::new(gicd_mmio_descriptor.start_addr().as_usize()),
            gicc: gicc::GICC::new(gicc_mmio_descriptor.start_addr().as_usize()),
            is_mmio_remapped: AtomicBool::new(false),
            handler_table: InitStateLock::new([None; Self::NUM_IRQS]),
        }
    }
}

//------------------
// OS Interface Code
//------------------

use synchronization::ReadWriteEx;

impl driver::DeviceDriver for GICv2 {
    fn compatible(&self) -> &'static str {
        "GICv2 (ARM Generic Interrupt Controller v2)"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let remapped = self.is_mmio_remapped.load(Ordering::Relaxed);
        if !remapped {
            let mut virt_addr = memory::mmu::kernel_map_mmio("GICD", &self.gicd_mmio_descriptor)?;
            self.gicd.set_mmio(virt_addr.as_usize());

            virt_addr = memory::mmu::kernel_map_mmio("GICC", &self.gicc_mmio_descriptor)?;
            self.gicc.set_mmio(virt_addr.as_usize());

            self.is_mmio_remapped.store(true, Ordering::Relaxed);
        }

        if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
            self.gicd.boot_core_init();
        }

        self.gicc.priority_accept_all();
        self.gicc.enable();

        Ok(())
    }
}

impl exception::asynchronous::interface::IRQManager for GICv2 {
    type IRQNumberType = IRQNumber;

    fn register_handler(
        &self,
        irq_number: Self::IRQNumberType,
        descriptor: exception::asynchronous::IRQDescriptor,
    ) -> Result<(), &'static str> {
        self.handler_table.write(|table| {
            let irq_number = irq_number.get();

            if table[irq_number].is_some() {
                return Err("IRQ handler already registered");
            }

            table[irq_number] = Some(descriptor);

            Ok(())
        })
    }

    fn enable(&self, irq_number: Self::IRQNumberType) {
        self.gicd.enable(irq_number);
    }

    fn handle_pending_irqs<'irq_context>(
        &'irq_context self,
        ic: &exception::asynchronous::IRQContext<'irq_context>,
    ) {
        let irq_number = self.gicc.pending_irq_number(ic);

        if irq_number > GICv2::MAX_IRQ_NUMBER {
            return;
        }

        self.handler_table.read(|table| {
            match table[irq_number] {
                None => panic!("No handler registered for IRQ {}", irq_number),
                Some(descriptor) => {
                    // Call the IRQ handler. Panics on failure.
                    descriptor.handler.handle().expect("Error handling IRQ");
                }
            }
        });

        self.gicc.mark_comleted(irq_number as u32, ic);
    }
}
