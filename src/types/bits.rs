// -------------------
// BITS AND BINARY CLASSES
// -------------------

// Prob best to just use bitfields

use alloc::{vec, vec::Vec};

// USEFUL CLASSES TO OPERATE ON BITS DIRECTLY
// for networking and non-byte like protocols

pub struct BinaryVal {
    content: Vec<bool>,
}

pub trait Condense {
    fn condense(&self) -> Vec<bool>;
}

impl Condense for bool {
    fn condense(&self) -> Vec<bool> {
        vec![*self]
    }
}

// anything that isnt 0 gets true else false
impl Condense for u64 {
    fn condense(&self) -> Vec<bool> {
        vec![*self != 0]
    }
}

impl BinaryVal {
    // accepts strings, arrays of ints/bools and lone int/bool with the trait Condense
    fn new<T: Condense>(args: T) -> BinaryVal {
        BinaryVal {
            content: args.condense(),
        }
    }
}

pub struct Bytes {
    pub content: Vec<u8>,
}

impl Bytes {
    pub fn from_bytes(args: &[u8]) -> Bytes {
        Bytes {
            content: args.to_vec(),
        }
    }

    pub fn from_int(i: u8) -> Bytes {
        Bytes { content: vec![i] }
    }
}
