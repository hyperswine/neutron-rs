#![cfg_attr(not(test), no_main)]
#![no_std]
#![feature(alloc_error_handler)]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![allow(named_asm_labels)]

// -----------------------
// RENDEVOUS POINT
// -----------------------

// After arch specific entry mechanisms, they should always end up calling _common

use neutron_kernel::memory::alloc::init_heap;

#[no_mangle]
extern "C" fn _common() {
    #[cfg(target_arch = "aarch64")]
    {
        neutron_kernel::arch::aarch64::console::basic_greet();
    }

    // INITIALISE KERNEL HEAP
    init_heap();

    // CREATE KERNEL_MANAGER AND LOAD _START
}

// -----------------------
// ARCBOOT CONFIG
// -----------------------

// An arcboot app is able to return
// arcboot_entry -> no mangles it. Basically main() but without rust doing weird things

// #[arcboot_entry]
// extern "C" fn arc_entry(arcservices: ArcServices) {
//     _common();
// }

// -----------------------
// LIMINE BOOT CONFIG
// -----------------------

#[cfg(feature = "limine")]
pub mod limine;

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

// required for main.rs
use core::{arch::asm, panic::PanicInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

// ----------
// TESTS
// ----------

// build with main
#[cfg(test)]
fn main() {}

// can also just test --lib
