// Pi 4B console facilities.

use core::fmt;

pub use cortex_a::asm::nop;

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        cortex_a::asm::wfe()
    }
}

#[cfg(not(feature = "test_build"))]
pub unsafe fn panic_console_out() -> impl fmt::Write {
    // NOTE: cpu::wait_forever actually refers to the generic cpu::wait_forever
    let gpio_mmio_start_addr = match super::GPIO.virt_mmio_start_addr() {
        None => wait_forever(),
        Some(x) => x,
    };

    let uart_mmio_start_addr = match super::PL011_UART.virt_mmio_start_addr() {
        None => wait_forever(),
        Some(x) => x,
    };

    let mut panic_gpio = PanicGPIO::new(gpio_mmio_start_addr);
    let mut panic_uart = PanicUart::new(uart_mmio_start_addr);

    panic_gpio
        .init(None)
        .unwrap_or_else(|_| wait_forever());
    panic_gpio.map_pl011_uart();
    panic_uart
        .init(None)
        .unwrap_or_else(|_| wait_forever());

    panic_uart
}

pub mod interface {
    use core::fmt;

    /// Console write functions.
    pub trait Write {
        /// Write a single character.
        fn write_char(&self, c: char);

        /// Write a Rust format string.
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        /// Block until the last buffered character has been physically put on the TX wire.
        fn flush(&self);
    }

    /// Console read functions.
    pub trait Read {
        /// Read a single character.
        fn read_char(&self) -> char {
            ' '
        }

        /// Clear RX buffers, if any.
        fn clear_rx(&self);
    }

    /// Console statistics.
    pub trait Statistics {
        /// Return the number of characters written.
        fn chars_written(&self) -> usize {
            0
        }

        /// Return the number of characters read.
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait All = Write + Read + Statistics;
}


// ?? Just returning a UART instance right
pub fn console() -> &'static impl interface::All {
    &super::PL011_UART
}
