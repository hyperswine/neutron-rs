// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

// BCM driver top level.

// TODO: load the interrupt controllers for pi3 on the fly with DTB or CSR

mod bcm2xxx_gpio;
// FOR Pi3 only
mod bcm2xxx_interrupt_controller;
mod bcm2xxx_pl011_uart;

pub use bcm2xxx_gpio::*;
// FOR Pi3 only
pub use bcm2xxx_interrupt_controller::*;
pub use bcm2xxx_pl011_uart::*;
