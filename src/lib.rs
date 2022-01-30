#![no_main]
#![no_std]
// works for all platforms
#![feature(alloc_error_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// TESTS
// #[cfg(feature = "artest")]
// mod artest;

#[cfg(feature = "arctest")]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}

// NON ARCH DEPENDENT CODE

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod process;
pub mod types;

// ! maybe doesnt expose to the rest of the modules unfortunately
extern crate alloc;
pub use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};

// ARCH DEPENDENT CODE

// Kernel Manager and ARCH Specific
pub mod kernel;
