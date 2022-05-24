#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(neutron_kernel::test_runner)]
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

    #[cfg(test)]
    test_main();

    // CREATE KERNEL_MANAGER AND LOAD _START
}

// -----------------------
// LIMINE BOOT CONFIG
// -----------------------

#[cfg(feature = "limine")]
use stivale_boot::{stivale2hdr, v2::*};

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
extern "C" fn entry_point(boot_info: &'static StivaleStruct) -> ! {
    boot_info.terminal().unwrap().term_write()("Hello, world!");

    _common();

    loop {}
}

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

// required for main.rs
use core::{arch::asm, panic::PanicInfo};

// If running the test config directly, use test_panic_handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    neutron_kernel::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
