// Pi 4B console facilities.

use core::fmt;

#[cfg(not(feature = "test_build"))]
pub unsafe fn panic_console_out() -> impl fmt::Write {
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

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}
