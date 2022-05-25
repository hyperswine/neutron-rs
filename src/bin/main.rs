#![no_main]
#![no_std]
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

// on x86, link to limine by default

// #[cfg(feature = "limine")]
// pub mod limine;

/*
use stivale_boot::{stivale2hdr, v2::*};

use crate::_common;

const STACK_SIZE: usize = 4096 * 16;

#[repr(C, align(4096))]
struct P2Align12<T>(T);
static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

static STIVALE_TERM: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static STIVALE_FB: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&STIVALE_TERM as *const StivaleTerminalHeaderTag).cast());

#[stivale2hdr]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK.0.as_ptr_range().end)
    .tags((&STIVALE_FB as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn limine_entry_point(boot_info: &'static StivaleStruct) -> ! {
    boot_info.terminal().unwrap().term_write()("Hello, world!");

    _common();

    loop {}
}

*/

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
