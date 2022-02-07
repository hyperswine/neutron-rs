#![cfg_attr(not(test), no_main)]
#![no_std]
#![feature(alloc_error_handler)]

// NON ARCH DEPENDENT CODE

use core::panic::PanicInfo;

#[cfg(not(test))]
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

