// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

// BSP console facilities.

use crate::{bsp::device_driver, console, cpu, driver};
use core::fmt;

// Public Code


#[cfg(not(feature = "test_build"))]
pub unsafe fn panic_console_out() -> impl fmt::Write {
    use driver::interface::DeviceDriver;

    // If remapping of the driver's MMIO hasn't already happened, we won't be able to print. Just
    // park the CPU core in this case.
    let gpio_mmio_start_addr = match super::GPIO.virt_mmio_start_addr() {
        None => cpu::wait_forever(),
        Some(x) => x,
    };

    let uart_mmio_start_addr = match super::PL011_UART.virt_mmio_start_addr() {
        None => cpu::wait_forever(),
        Some(x) => x,
    };

    let mut panic_gpio = device_driver::PanicGPIO::new(gpio_mmio_start_addr);
    let mut panic_uart = device_driver::PanicUart::new(uart_mmio_start_addr);

    panic_gpio
        .init(None)
        .unwrap_or_else(|_| cpu::wait_forever());
    panic_gpio.map_pl011_uart();
    panic_uart
        .init(None)
        .unwrap_or_else(|_| cpu::wait_forever());

    panic_uart
}


#[cfg(feature = "test_build")]
pub unsafe fn panic_console_out() -> impl fmt::Write {
    use driver::interface::DeviceDriver;

    let uart_mmio_start_addr = match super::PL011_UART.virt_mmio_start_addr() {
        None => cpu::wait_forever(),
        Some(x) => x,
    };
    let mut panic_uart = device_driver::PanicUart::new(uart_mmio_start_addr);

    panic_uart
        .init(None)
        .unwrap_or_else(|_| cpu::qemu_exit_failure());

    panic_uart
}


pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}

// Testing


#[cfg(feature = "test_build")]
pub fn qemu_bring_up_console() {
    use driver::interface::DeviceDriver;

    // Calling the UART's init ensures that the BSP's instance of the UART does remap the MMIO
    unsafe {
        super::PL011_UART
            .init()
            .unwrap_or_else(|_| cpu::qemu_exit_failure());
    }
}
