#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
// for x86 test runner
#![cfg_attr(target_arch="x86_64", test_runner(crate::test_runner))]

// NON ARCH DEPENDENT CODE

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    write_uart!("Running tests");
    for test in tests {
        test.run();
    }
    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    write_uart!("[failed]\n");
    write_uart!("Error: {}\n", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
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
