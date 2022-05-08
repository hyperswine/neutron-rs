use core::fmt;

// ACTUAL CONSOLE

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    #[cfg(target_arch = "aarch64")]
    use console::interface::Write;

    // FOR RISCV Here (UART)

    console::console().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print::_print(format_args_nl!($($arg)*));
    })
}

#[macro_export]
macro_rules! info {
    ($string:expr) => ({
        use $crate::time::interface::TimeManager;

        let timestamp = $crate::time::time_manager().uptime();

        $crate::print::_print(format_args_nl!(
            concat!("[  {:>3}.{:06}] ", $string),
            timestamp.as_secs(),
            timestamp.subsec_micros(),
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        use $crate::time::interface::TimeManager;

        let timestamp = $crate::time::time_manager().uptime();

        $crate::print::_print(format_args_nl!(
            concat!("[  {:>3}.{:06}] ", $format_string),
            timestamp.as_secs(),
            timestamp.subsec_micros(),
            $($arg)*
        ));
    })
}

#[macro_export]
macro_rules! warn {
    ($string:expr) => ({
        use $crate::time::interface::TimeManager;

        let timestamp = $crate::time::time_manager().uptime();

        $crate::print::_print(format_args_nl!(
            concat!("[W {:>3}.{:06}] ", $string),
            timestamp.as_secs(),
            timestamp.subsec_micros(),
        ));
    });
    ($format_string:expr, $($arg:tt)*) => ({
        use $crate::time::interface::TimeManager;

        let timestamp = $crate::time::time_manager().uptime();

        $crate::print::_print(format_args_nl!(
            concat!("[W {:>3}.{:06}] ", $format_string),
            timestamp.as_secs(),
            timestamp.subsec_micros(),
            $($arg)*
        ));
    })
}
