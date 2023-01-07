#![no_std]
#![no_main]

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

#[no_mangle]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "
        la sp, stack
        li a0, 1024*4
        csrr a1, mhartid
        addi a1, a1, 1
        mul a0, a0, a1
        add sp, sp, a0
    "
        );
    }

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
