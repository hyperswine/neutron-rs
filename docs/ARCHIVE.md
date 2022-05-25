# Archived Code

Just a bunch of code that I cut out to use later.

```rust
use core::fmt;

// ACTUAL CONSOLE

// #[doc(hidden)]
// pub fn _print(args: fmt::Arguments) {
//     #[cfg(target_arch = "aarch64")]
//     use console::interface::Write;

//     // FOR RISCV Here (UART)

//     console::console().write_fmt(args).unwrap();
// }

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
// }

// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ({
//         $crate::print::_print(format_args_nl!($($arg)*));
//     })
// }

// println! just prints to stdout in userspace
// in kernelspace, just print to the default buffer, aka the arch's preferred default stream like UART0

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
```

## Limine

I took out:

```rust
// -----------------------
// LIMINE BOOT CONFIG
// -----------------------

// on x86, link to limine by default

// #[cfg(feature = "limine")]
// pub mod limine;

/*
use stivale_boot::{stivale2hdr, v2::*};

use crate::_common;

const STACK_SIZE: usize = 4096 * 16;

#[repr(C, align(4096))]
struct P2Align12<T>(T);
static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

static STIVALE_TERM: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static STIVALE_FB: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&STIVALE_TERM as *const StivaleTerminalHeaderTag).cast());

#[stivale2hdr]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK.0.as_ptr_range().end)
    .tags((&STIVALE_FB as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn limine_entry_point(boot_info: &'static StivaleStruct) -> ! {
    boot_info.terminal().unwrap().term_write()("Hello, world!");

    _common();

    loop {}
}

*/
```
