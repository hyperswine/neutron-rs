// EXPERIMENTAL x86 SUPPORT FOR EASE OF TESTING ON QEMU

// A lot of nice crates on x86_64 we could rip off, I meant use

pub mod virtual_memory;
pub mod exception;

// TODO: memory allocator
// TODO: boot entry asm and linker script

// use alloc::alloc::{GlobalAlloc, Layout};
// use core::ptr::null_mut;

// struct OptimalAllocator;

// unsafe impl GlobalAlloc for OptimalAllocator {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
//         null_mut()
//     }

//     unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//         panic!("dealloc should be never called")
//     }
// }

// #[global_allocator]
// static ALLOCATOR: OptimalAllocator = OptimalAllocator;

// #[alloc_error_handler]
// fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
//     panic!("allocation error: {:?}", layout)
// }

pub const HEAP_START: usize = 0x_4444_4444_0000;
// 100 KiB by default for the kernel. For programs, idk
pub const HEAP_SIZE: usize = 100 * 1024; 
