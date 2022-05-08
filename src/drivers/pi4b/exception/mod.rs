// Pi 4B asynchronous exception handling.

pub const PL011_UART: IRQNumber = IRQNumber::new(153);

pub fn irq_manager() -> &'static impl exception::asynchronous::interface::IRQManager<
    IRQNumberType = IRQNumber,
> {
    &super::super::INTERRUPT_CONTROLLER
}
