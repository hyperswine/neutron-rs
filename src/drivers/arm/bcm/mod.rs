// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

// BCM driver top level.

// BCM SoCs are ubiquitous in Pis

mod bcm2xxx_gpio;
pub use bcm2xxx_pl011_uart::*;
mod bcm2xxx_pl011_uart;
pub use bcm2xxx_gpio::*;

// FOR Pi3 only
mod bcm2xxx_interrupt_controller;
// FOR Pi3 only
pub use bcm2xxx_interrupt_controller::*;
