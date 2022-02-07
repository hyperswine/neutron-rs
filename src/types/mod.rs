pub mod ops;

// RISCV
#[cfg(not(test))]
use alloc::{string::String, vec, vec::Vec};
use core::option::Option;

// USEFUL CLASSES TO OPERATE ON BITS DIRECTLY

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

// TIME

pub struct KTimestamp {
    day: u8,
    month: u8,
    year: u8,

    hour: f32,
    min: f32,
    sec: f32,
}

impl KTimestamp {
    // yyyy-mm-dd
    pub fn new(str: &str) -> Self {
        // TODO check if in right format
        // if str.len() != 10 {
        //     return Option::None;
        // }

        Self {
            day: 1,
            month: 1,
            year: 1,
            hour: 1.0,
            min: 1.0,
            sec: 1.0,
        }
    }
}
