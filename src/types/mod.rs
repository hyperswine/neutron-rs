pub mod ops;
pub mod bits;

use alloc::{string::String, vec, vec::Vec};
use core::option::Option;

// -------------------
// Useful base classes to subclass
// -------------------

// Data Structures

// Useful for comparable (<) data
pub struct KPriorityNode<T: PartialOrd> {
    next: *mut KPriorityNode<T>,
    t: T
}

pub struct KPriorityQueue<T: PartialOrd> {
    head: KPriorityNode<T>,
    n_elem: usize,
}

impl<T: PartialOrd> KPriorityNode<T> {
    pub fn new(t: T) -> Self {
        use core::ptr::null_mut;
        Self { next: null_mut(), t }
    }
}

impl<T: PartialOrd> KPriorityQueue<T> {
    pub fn new(t: T) -> Self {
        Self {
            head: KPriorityNode::new(t),
            n_elem: 1,
        }
    }
    pub fn queue_node(&mut self, node: KPriorityNode<T>) {
        self.head;
    }
}

// requires underlying RNG, assuming exists in pi4/spectro in the drivers::Random function
pub struct KMap;

// generic node
struct Node;

struct Queue;
struct PriorityQueue;
struct HeapQueue;

struct Stack;
struct Heap;

struct BinaryTree;
// should be used for neutron filesystem
struct BalanceTree;

pub trait Search {
    fn breadth_first_search();
    fn depth_first_search();
}

// -------------------
// Classes for specialised algorithms
// -------------------

struct Bucket;

struct SkipList;

// -------------------
// Direct VGA Render (shell like)
// -------------------

// renders to the framebuffer using the cpu

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// stores all the chars that have been written to it
struct ShellBuffer;

// the resolution framebuffer that is refreshed (usually rewritten entirely) on every write
// to shell buffer
struct Framebuffer;

type Path = [u8; 300];

type Font = u64;

// a shell is a single process with its own path and ELF virtual memory

// when kernel starts, it
// start a shell with a given pid
// and render it as the default shell
pub struct Shell {
    resolution: (u64, u64),
    font: Font,
    color: Color,
}

pub trait ShellFunctions {
    fn new(res: (u64, u64)) -> Self;
    fn write(&self, _str: &str);
    fn writeln(&self, _str: &str);
    fn scroll_y(&self, offset: u64);
    fn scroll_x(&self, offset: u64);
    fn use_font(&self, font: Font);
    fn use_color(&self, color: Color);
}



// -------------------
// TIME
// -------------------

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
