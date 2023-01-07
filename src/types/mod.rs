pub mod bits;
pub mod synchronisation;
pub mod sort;
pub mod time;
pub mod paging;
// expose time functions
pub use time::*;

use alloc::{
    borrow::ToOwned,
    fmt,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::option::Option;

// -------------------
// Useful base classes to subclass
// -------------------

// Data Structures

// Useful for comparable (<) data
pub struct KPriorityQueue<T: Ord> {
    queue: Vec<T>,
    size: usize,
}

impl<T: Ord> KPriorityQueue<T> {
    // default constructor
    pub fn new() -> Self {
        Self {
            queue: vec![],
            size: 0,
        }
    }

    // insert data in order
    pub fn insert_data(&mut self, data: T) {
        let pos = self.queue.binary_search(&data).unwrap_or_else(|e| e);
        self.queue.insert(pos, data);
    }

    // pop from front of the queue
    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

// requires underlying RNG, assuming exists in pi4/spectro in the drivers::Random function
pub struct KMap;

// generic node
struct KQueue;
struct KHeapQueue;

struct KStack;
struct KHeap;

struct KBinaryTree;
// should be used for neutron filesystem
struct KBalanceTree;

pub trait Search {
    fn breadth_first_search();
    fn depth_first_search();
}

// -------------------
// Classes for specialised algorithms
// -------------------

struct Bucket;

struct SkipList;

// ----------------
// ALIGNMENT
// ----------------

/// Check if a value is aligned to a given size.
#[inline(always)]
pub const fn is_aligned(value: usize, alignment: usize) -> bool {
    assert!(alignment.is_power_of_two());

    (value & (alignment - 1)) == 0
}

/// Align down.
#[inline(always)]
pub const fn align_down(value: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());

    value & !(alignment - 1)
}

/// Align up.
#[inline(always)]
pub const fn align_up(value: usize, alignment: usize) -> usize {
    assert!(alignment.is_power_of_two());

    (value + alignment - 1) & !(alignment - 1)
}
