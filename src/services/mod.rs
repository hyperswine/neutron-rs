pub mod manager;

pub struct MemoryBlock {
    mem: *mut u8,
    size_bytes: u64,
}

pub trait Service {
    // request this service from the kernel
    fn request();
    // receive the result in the form of a readable buffer/memory block/pages
    fn receive() -> MemoryBlock;
}

use core::ptr;

// very basic print, does not support formatting
// must write "\n" in addition to your <"string"> enclosed within double quotes
#[macro_export]
macro_rules! print {
    ($a:expr) => {
        const UART0: *mut u8 = 0x10000000 as *mut u8;
        // ! need to cast as byte instead?
        let out_str = b"$a";
        for byte in out_str {
            unsafe {
                ptr::write_volatile(UART0, *byte);
            }
        }
    }
}
