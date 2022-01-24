#![no_std]
#![no_main]

// should include all the modules here for no_std to work properly I think?
// then can remove the no_main from the arch depedent code entry point

// #[cfg(target_arch = "riscv64")]

// works well if you are testing for a single platform I guess, but we arent
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

// for tests to work, you have to compile and link to the final library, i.e. specify the module here for local unittests to work
// pub mod kernel;
// pub mod filesystem;
// pub mod types;

// IDEA: compile the kernel to a bare static library for whatever arch you want
// Then link to the bootloader binary for the arch you want to create a bootloader + kernel img

use core::panic::PanicInfo;
use core::ptr;

// extern crate alloc;
// use alloc::string::String;
// use alloc::vec;

// use crate::filesystem::{Filesystem, File};

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for the Kernel
#[cfg(target_arch = "riscv64")]
pub mod kernel;

// Entry point for x86 (testing purposes)
#[cfg(target_arch = "x86_64")]
fn main() {

}

// does this work with no std?
// #[cfg(test)]
// extern crate std;

// TESTS
#[cfg(test)]
fn t_log(out_str: &[u8]) {
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    // let out_str = b"succesfully loaded _start() on bare metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}

#[test]
fn trivial_assertion() {
    t_log(b"trivial assertion");
    assert_eq!(1, 1);
    t_log(b"ok");
}
