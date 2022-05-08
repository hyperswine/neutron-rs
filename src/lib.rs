#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(alloc_error_handler)]
#![allow(named_asm_labels)]
#![feature(global_asm)]
#![feature(asm_const)]

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

pub mod drivers;
pub mod filesystem;
pub mod kmod;
pub mod process;
pub mod services;
pub mod types;
pub mod memory;
pub mod exception;

// -----------------------
// ARCH DEPENDENT CODE
// -----------------------

extern crate alloc;
extern crate goblin;

use core::{fmt, panic::PanicInfo};

use alloc::string::String;

// Kernel Manager and ARCH Specific
pub mod kernel;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    let mut i = 0;

    for test in tests {
        // I THINK IT PANICS HERE SINCE ALLOCATOR ISNT SET UP PROPERLY
        // CAN JUST USE STACK SOMEHOW
        println!("Running Test {}", i);
        test.run();
        i = i + 1;
        write_uart!(b"Test [Passed]!\n\n");
    }
    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    write_uart!(b"Test [Failed]\n");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// FOR INTEGRATION TESTING ONLY, basic boilerplate _start for all arch
// For unit tests, just #[test_case] will do
#[no_mangle]
#[cfg(test)]
pub extern "C" fn _start() -> ! {
    #[cfg(target_arch = "riscv64")]
    {
        use kernel::arch::riscv64gc::init_uart;
        init_uart();
    }

    // all arches should export write_uart
    write_uart!(b"Running Test Config...\n");

    test_main();

    loop {}
}
