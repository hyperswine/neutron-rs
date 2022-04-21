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
    let heap_start = HEAP_START;
    let heap_end = HEAP_START - HEAP_SIZE;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
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
pub const HEAP_START: usize = 0x100000;
// 100 KiB by default for the kernel. For programs, idk
pub const HEAP_SIZE: usize = 100 * 1024;

// TODO: Implement fixed allocator
struct FixedAllocator;
