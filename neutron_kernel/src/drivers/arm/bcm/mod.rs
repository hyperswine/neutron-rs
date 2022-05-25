// BCM driver top level.

mod bcm2xxx_gpio;
pub use bcm2xxx_pl011_uart::*;
mod bcm2xxx_pl011_uart;
pub use bcm2xxx_gpio::*;

// FOR Pi3 only
mod bcm2xxx_interrupt_controller;
// FOR Pi3 only
pub use bcm2xxx_interrupt_controller::*;
