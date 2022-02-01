#![no_main]
#![no_std]
// works for all platforms
#![feature(alloc_error_handler)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

// TESTS
#[cfg(feature = "arctest")]
mod arctest;

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
