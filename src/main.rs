#![no_main]
#![no_std]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![allow(named_asm_labels)]

use core::{arch::asm, panic::PanicInfo};
use neutron_kernel::arch::riscv64gc::begin_riscv;

// -----------------------
// RENDEVOUS POINT
// -----------------------

#[no_mangle]
extern "C" fn start() -> ! {
    #[cfg(target_arch = "riscv64")]
    begin_riscv();

    loop {}
}

// -----------------------
// ARCBOOT CONFIG
// -----------------------

#[cfg(feature = "arcboot")]
pub mod arcboot;

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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
