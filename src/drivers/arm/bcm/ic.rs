// Interrupt Controller Driver.

mod peripheral_ic;

struct PendingIRQs {
    bitmask: u64,
}

pub type LocalIRQ =
    exception::asynchronous::IRQNumber<{ InterruptController::MAX_LOCAL_IRQ_NUMBER }>;
pub type PeripheralIRQ =
    exception::asynchronous::IRQNumber<{ InterruptController::MAX_PERIPHERAL_IRQ_NUMBER }>;

#[derive(Copy, Clone)]
pub enum IRQNumber {
    Local(LocalIRQ),
    Peripheral(PeripheralIRQ),
}

pub struct InterruptController {
    periph: peripheral_ic::PeripheralIC,
}

impl PendingIRQs {
    pub fn new(bitmask: u64) -> Self {
        Self { bitmask }
    }
}

impl Iterator for PendingIRQs {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        use core::intrinsics::cttz;

        let next = cttz(self.bitmask);
        if next == 64 {
            return None;
        }

        self.bitmask &= !(1 << next);

        Some(next as usize)
    }
}

impl InterruptController {
    const MAX_LOCAL_IRQ_NUMBER: usize = 11;
    const MAX_PERIPHERAL_IRQ_NUMBER: usize = 63;
    const NUM_PERIPHERAL_IRQS: usize = Self::MAX_PERIPHERAL_IRQ_NUMBER + 1;

    pub const unsafe fn new(
        _local_mmio_descriptor: memory::mmu::MMIODescriptor,
        periph_mmio_descriptor: memory::mmu::MMIODescriptor,
    ) -> Self {
        Self {
            periph: peripheral_ic::PeripheralIC::new(periph_mmio_descriptor),
        }
    }
}

impl driver::interface::DeviceDriver for InterruptController {
    fn compatible(&self) -> &'static str {
        "BCM Interrupt Controller"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.periph.init()
    }
}

impl IRQManager for InterruptController {
    type IRQNumberType = IRQNumber;

    fn register_handler(
        &self,
        irq: Self::IRQNumberType,
        descriptor: IRQDescriptor,
    ) -> Result<(), &'static str> {
        match irq {
            IRQNumber::Local(_) => unimplemented!("Local IRQ controller not implemented."),
            IRQNumber::Peripheral(pirq) => self.periph.register_handler(pirq, descriptor),
        }
    }

    fn enable(&self, irq: Self::IRQNumberType) {
        match irq {
            IRQNumber::Local(_) => unimplemented!("Local IRQ controller not implemented."),
            IRQNumber::Peripheral(pirq) => self.periph.enable(pirq),
        }
    }

    fn handle_pending_irqs<'irq_context>(&'irq_context self, ic: &IRQContext<'irq_context>) {
        // It can only be a peripheral IRQ pending because enable() does not support local IRQs yet.
        self.periph.handle_pending_irqs(ic)
    }

    fn print_handler(&self) {
        self.periph.print_handler();
    }
}
