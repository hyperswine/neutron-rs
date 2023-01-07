// ------------
// IRQ
// ------------

// Service handlers should make use of these functions
// Or maybe just get register themselves with the syscall handlers

pub trait IRQHandler {
    fn handle(&self) -> Result<(), &'static str>;
}

/// Arch specific code should register its own IRQ handler
pub trait IRQManager {
    fn register_handler(
        &self,
        irq_number: IRQNumber,
    ) -> Result<(), &'static str>;

    fn enable(&self, irq_number: IRQNumber);

    fn handle_pending_irqs<'irq_context>(&'irq_context self);

    fn print_handler(&self);
}

pub struct IRQNumber(usize);
