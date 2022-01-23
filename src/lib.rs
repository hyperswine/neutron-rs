#![no_std]
#![no_main]

pub mod kernel;
pub mod filesystem;

// IDEA: compile the kernel to a bare static library for whatever arch you want
// Then link to the bootloader binary for the arch you want to create a bootloader + kernel img

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point for the Kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"succesfully loaded _start() on bare metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }

    // create kernel
    let kern_manager = kernel::KernelManager;

    // CALL kernel_main()

    // call clean_up() to write all pending operations to disk

    // loop for now so the function wont return (later can make it 'return' to bare metal aka exit/stop execution completely without an error code)
    loop {}
}
