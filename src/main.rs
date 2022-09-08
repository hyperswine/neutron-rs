#![no_main]
#![no_std]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![allow(named_asm_labels)]

// -----------------------
// RENDEVOUS POINT
// -----------------------

// BOOT FLOW:
// ? setup higher half kernel with #define if linking to C with limine, or assume if arcboot
// for rust, maybe specify it in the linker script
// Arch specific code jumps on, does its thing, calls common code
// Pass off to common entry point

// After arch specific entry mechanisms, they should always end up calling common(), which starts up the real initialisation of drivers and modules and subsystems

// technically dont have to use _start, just need a linker script to specify a custom entry point just the entry point of choice if no feature flag for arcboot or other bootloaders are done
// _start always exists, but might not be the actual entry point

#[no_mangle]
extern "C" fn _start() -> ! {
    loop {}
}

// -----------------------
// ARCBOOT CONFIG
// -----------------------

use arcboot_api::ArcServices;
use neutron_kernel::{arch::aarch64::entry::arch_init, kernel::common};

// An arcboot app is able to return
// arcboot_entry -> no mangles it. Basically main() but without rust doing weird things
// Cant be bothered writing an [arc_entry] macro

extern "C" fn arc_entry(arcservices: ArcServices) {
    #[cfg(target_arch = "aarch64")]
    arch_init(arcservices);

    // SHOULD BE CALLED BY THE ARCH INIT CODE, or maybe after the arch init code, it returns here
    common();
}

// -----------------------
// STIVALE CONFIG
// -----------------------

// NOW: these symbols will still exist, but they are irrelevant
// I could put it in a mod or another binary, but uh..

#[macro_use]
extern crate stivale_boot;

use stivale_boot::v2::*;

#[repr(C, align(4096))]
struct P2Align12<T>(T);

const STACK_SIZE: usize = 4096 * 16;

static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

static STIVALE_TERM: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static STIVALE_FB: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&STIVALE_TERM as *const StivaleTerminalHeaderTag).cast());

#[stivale2hdr]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK.0.as_ptr_range().end)
    .tags((&STIVALE_FB as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn stivale_main(boot_info: &'static StivaleStruct) -> ! {
    boot_info.terminal().unwrap().term_write()("Hello, world!");

    loop {}
}

// -----------------------
// LIMINE CONFIG
// -----------------------

use limine::*;

static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);
static MMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);

#[no_mangle]
extern "C" fn limine_main() -> ! {

    let bootloader_info = BOOTLOADER_INFO
        .get_response()
        .get()
        .expect("barebones: recieved no bootloader info");

    let mmap = MMAP
        .get_response()
        .get()
        .expect("barebones: recieved no mmap")
        .memmap();

    loop {}
}

// -----------------
// MULTIBOOT
// -----------------

/*
.section .multiboot_header
header_start:
    .quad 0xe85250d6
    .quad 0
    .quad header_end - header_start
    .quad 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    .word 0
    .word 0
    .quad 8
header_end:
*/

struct MultibootHeaderSpec2 {
    magic: u128,
    zero: u128,
    size: u128,
    align_constant: u128,
    zero_two: u32,
    zero_three: u32,
    eight: u128,
}

// const NEUTRON_MULTIBOOT_HEADER: MultibootHeaderSpec = MultibootHeaderSpec2 { magic: 0xe85250d6 };

// -----------------------
// AUXILIARY CODE
// -----------------------

// required for main.rs
use core::{arch::asm, panic::PanicInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
