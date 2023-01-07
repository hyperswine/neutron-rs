#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::abort;
use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use core::panic::PanicInfo;

use fdt::Fdt;

const UART: *mut u8 = 0x09000000 as *mut u8;

fn putchar(c: u8) {
    unsafe { *UART = c };
}

fn print(s: &str) {
    for c in s.chars() {
        putchar(c as u8);
    }
}

fn read_char() -> u8 {
    unsafe {
        while volatile_load(0x3F201018 as *const u32) & (1 << 4) > 0 {}
        volatile_load(0x3F201000 as *const u32) as u8
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "
            ldr x5, =_start
            mov sp, x5
            ldr x30, =stack_top
            mov sp, x30
            ldr     x5, =__bss_start
            ldr     w6, =__bss_size
            1:  cbz     w6, 2f
                str     xzr, [x5], #8
                sub     w6, w6, #1
                cbnz    w6, 1b
            2:
        "
        );
    }

    print("Hello Rust!\n");

    // read fdt at 0x4000_0000
    unsafe {
        match Fdt::from_ptr(0x4000_0000 as *const u8) {
            Ok(f) => {
                print("FDT FOUND AT 0x4000_0000")
            },
            Err(e) => print("FDT NOT FOUND AT 0x4000_0000..."),
        }
    }

    loop {
        // putchar(read_char())
    }
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

#[cfg(not(test))]
#[lang = "eh_personality"]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    loop {}
}
