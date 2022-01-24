pub mod vec;

use core::fmt::Binary;

// use crate::drivers::vga_buffer::klog;

// stores an array of bools in low-index is high format, i.e. [0] stores the most significant bit and [n-1] is the least significant bit

/*
pub struct Binary {
    content: Vec<bool>
}

trait Condense {
    fn condense(&self) -> Vec<bool>;
}

impl Condense for bool {
    fn condense(&self) -> Vec<bool> {
        vec!(&self)
    }
}

// anything that isnt 0 gets true else false
impl Condense for u64 {
    fn condense(&self) -> Vec<bool> {
        vec!(&self != 0)
    }
}

impl Condense for Vec<u64> {
    fn condense(&self) -> Vec<bool> {
        vec!(&self)
    }
}

impl Binary {
    // accepts strings, arrays of ints/bools and lone int/bool with the trait Condense
    fn new<A>(args: A) -> Binary {
        Binary{content: args.condense()}
    }
}

pub struct Byte {
    content: Vec<u8>
}

impl Byte {
    fn new(args: [u8]) -> Binary {
        Binary{content: Vec<u8>::new(args)}
    }
}

pub struct Size;
*/
