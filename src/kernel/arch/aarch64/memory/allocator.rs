use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

// -----------------
// OPTIMAL ALLOCATOR
// -----------------

// Change this to suit kernel needs
type OptimalAllocator = FixedAllocator;

// -----------------
// FIXED ALLOCATOR
// -----------------

// Arbitary starting address
pub const HEAP_START: usize = 0x_4444_4444_0000;
// 100 KiB by default for the kernel. For programs, idk
pub const HEAP_SIZE: usize = 100 * 1024; 

// TODO: Implement fixed allocator
struct FixedAllocator;
