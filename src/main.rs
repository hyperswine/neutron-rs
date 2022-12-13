#![no_main]
#![no_std]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![allow(named_asm_labels)]

// -----------------------
// RENDEVOUS POINT
// -----------------------

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

#[cfg(feature = "stivale")]
pub mod stivale;

// -----------------------
// LIMINE CONFIG
// -----------------------

#[cfg(feature = "limine")]
pub mod limine_boot;

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
