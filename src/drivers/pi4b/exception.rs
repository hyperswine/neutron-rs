// Pi 4B asynchronous exception handling.

use crate::exception::{IRQNumber, interface::IRQManager};
use super::INTERRUPT_CONTROLLER;

pub const PL011_UART: IRQNumber = IRQNumber::new(153);

pub fn irq_manager() -> &'static impl IRQManager<
    IRQNumberType = IRQNumber,
> {
    &INTERRUPT_CONTROLLER
}
