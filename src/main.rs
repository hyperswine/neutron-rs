#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

// NON ARCH DEPENDENT CODE

extern crate alloc;

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// FOR TESTING ONLY, basic boilerplate _start for all arch
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    // all arches should export write_uart
    write_uart!("Running Test Config...");

    test_main();

    loop {}
}

pub mod drivers;
pub mod filesystem;
pub mod kext;
pub mod process;
pub mod services;
pub mod types;

// ARCH DEPENDENT CODE

// Kernel Manager and ARCH Specific
pub mod kernel;
