#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
// TESTS
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// NON ARCH DEPENDENT CODE

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod process;
pub mod types;
pub mod services;

// stephen's implementation, seems to have a few issues
// pub mod stephen;

// ! maybe doesnt expose to the rest of the modules unfortunately
extern crate alloc;
pub use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};

// ARCH DEPENDENT CODE

// Kernel Manager and ARCH Specific
pub mod kernel;

// RISC V

use core::ptr;

// Entry point for the Kernel
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // hook onto the start function to when testing, else ignore when building the final code
    #[cfg(test)]
    test_main();

    // exit after testing
    // #[cfg(test)]
    // exit(0);

    // create kernel
    // let kern_manager = kernel::KernelManager::new();
    // CALL kernel_main()

    // call clean_up() to write all pending operations to disk

    // loop for now so the function wont return (later can make it 'return' to bare metal aka exit/stop execution completely without an error code)
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    // #[macro_use]
    // use crate::println;
    // println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
