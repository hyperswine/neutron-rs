#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" fn _entry() -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}
