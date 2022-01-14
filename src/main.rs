#![no_std]
#![no_main]

// import panic handler for the final binary
use neutronkern::panic;

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