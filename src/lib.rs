#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;


#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
    Entry point for the Kernel
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"riscv64 bare metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // VGA
    loop {}
}


