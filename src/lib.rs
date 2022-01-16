#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
    Entry point for the Kernel
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/*
    Setup interfaces for filesystem, memory, networking, etc
*/