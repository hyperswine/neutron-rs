#![no_main]
#![no_std]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![allow(named_asm_labels)]

// -----------------------
// RENDEVOUS POINT
// -----------------------

// After arch specific entry mechanisms, they should always end up calling common(), which creates a kernel manger
// and a bunch of other things

fn common() {}

// technically dont have to use _start, just need a linker script to specify a custom entry point
// just the entry point of choice if no feature flag for arcboot or other bootloaders are done

#[no_mangle]
extern "C" fn _start() -> ! {
    loop {}
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
// NON ARCH DEPENDENT CODE
// -----------------------

// required for main.rs
use core::{arch::asm, panic::PanicInfo};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
