// should only be defined once when building for a specific arch
// if just testing out functions, test the functions directly without this module
#![feature(alloc_error_handler)]

// list allocator and fixed size allocator + variants
pub mod list;
pub mod fixed;

// have to enable feature. Do it for riscv64 only
use alloc::alloc::{GlobalAlloc, Layout};

struct OptimalAllocator;

impl GlobalAllocator for OptimalAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

// also maybe have cfg for allocator types to compile and try out
// allocator_optimal, allocator_fixedsize
// NOTE: automatically implements Box, Vec, Rc once you have a global allocator
// basically just assume this works for now and use Box, Vec, Rc freely. Just need to test those functions anyway
#[global_allocator]
static ALLOCATOR: OptimalAllocator = OptimalAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
// 100 KiB by default for the kernel. For programs, idk
pub const HEAP_SIZE: usize = 100 * 1024; 
