// BCM driver top level.

mod gpio;
mod uart;

// FOR Pi3 only
mod bcm2xxx_interrupt_controller;
// FOR Pi3 only
pub use bcm2xxx_interrupt_controller::*;
