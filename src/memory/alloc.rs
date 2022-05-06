use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// -----------------
// LINKED LIST ALLOCATOR
// -----------------

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

// -----------------
// Memory Mapped Files
// -----------------

pub const MMIO_START: usize = 0x7fff000000000000;

// -----------------
// OPTIMAL ALLOCATOR
// -----------------

// Change this to suit kernel needs
type OptimalAllocator = FixedAllocator;

// -----------------
// FIXED ALLOCATOR
// -----------------

// Arbitary starting address (vaddr)
pub const HEAP_START: usize = 0x8000_0000;
// 4 * 4 KiB by default for the kernel. For programs, idk
pub const HEAP_SIZE: usize = 4 * 0x1000;

// TODO: Implement fixed allocator
struct FixedAllocator;
