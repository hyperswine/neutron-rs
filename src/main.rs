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
use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader};

static STACK: [u8; 4096] = [0; 4096];

static FRAMEBUFFER_TAG: StivaleFramebufferHeaderTag =
    StivaleFramebufferHeaderTag::new().framebuffer_bpp(24);

#[link_section = ".stivale2hdr"]
#[no_mangle]
#[used]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(&STACK[4095] as *const u8)
    .tags((&FRAMEBUFFER_TAG as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn entry_point(_header_addr: usize) -> ! {
    for mut char in b"Hello, World !".iter() {
        unsafe {
            let mut port = 0x3F8;
            // IDK if out() or in()
            // asm!("outb {}, {}", out(reg) port, out(reg) char);
        }
    }

    _common();

    loop {}
}

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

// required for main.rs
use core::{panic::PanicInfo, arch::asm};

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
