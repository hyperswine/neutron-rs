pub mod ops;

// use crate::drivers::vga_buffer::klog;

// stores an array of bools in low-index is high format, i.e. [0] stores the most significant bit and [n-1] is the least significant bit

// TEST
#[cfg(test)]
use std::{vec, vec::Vec};

pub struct BinaryVal {
    content: Vec<bool>
}

pub trait Condense {
    fn condense(&self) -> Vec<bool>;
}

impl Condense for bool {
    fn condense(&self) -> Vec<bool> {
        vec!(*self)
    }
}

// anything that isnt 0 gets true else false
impl Condense for u64 {
    fn condense(&self) -> Vec<bool> {
        vec!(*self != 0)
    }
}

// impl Condense for Vec<u64> {
//     fn condense(&self) -> Vec<bool> {
//         vec!(*self)
//     }
// }

impl BinaryVal {
    // accepts strings, arrays of ints/bools and lone int/bool with the trait Condense
    fn new<T: Condense>(args: T) -> BinaryVal {
        BinaryVal{content: args.condense()}
    }
}

pub struct Bytes {
    pub content: Vec<u8>
}

// use self::ops::Range;

// impl Range for Bytes {
//     type Output<'a> = &'a[u8];
//     fn range(self, start_inc: u64, end_exc: u64) -> Self::Output {
//         self.content[start_inc..end_exc]
//     }
// }

impl Bytes {
    pub fn from_bytes(args: &[u8]) -> Bytes {
        Bytes{content: Vec::from(args)}
    }

    pub fn from_int(i: u8) -> Bytes {
        Bytes{content: vec!(i)}
    }
}

// pub struct Size;
