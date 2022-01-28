#![no_main]
#![no_std]

// TESTS
#[cfg(test)]
mod test;

// ARCH DEPENDENT CODE

// Kernel Manager and ARCH Specific
pub mod kernel;

// NON ARCH DEPENDENT CODE

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub mod process;
pub mod types;

// doesnt expose to the rest of the modules unfortunately
extern crate alloc;
pub use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};
