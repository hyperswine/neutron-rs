pub mod bits;
pub mod ops;

use alloc::{string::String, vec, vec::Vec};
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
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// stores all the chars that have been written to it
struct ShellBuffer {
    buffer: String,
    size_bytes: usize,
    n_lines: usize,
}

// the resolution framebuffer that is refreshed (usually rewritten entirely) on every write the to shell buffer
struct Framebuffer {
    framebuffer_addr: u64,
}

type Path = String;

// * for now, a font is simply the path to the font, implemented by neutron
type Font = Path;

type Resolution = (u64, u64);

// a shell is a single process with its own path and ELF virtual memory

// when kernel starts, it
// start a shell with a given pid
// and render it as the default shell
pub struct Shell {
    resolution: Resolution,
    font: Font,
    color: ColorCode,
}

pub trait ShellFunctions {
    fn write(&self, _str: &str);
    fn writeln(&self, _str: &str);
    fn scroll_y(&self, offset: u64);
    fn scroll_x(&self, offset: u64);
    fn use_font(&self, font: Font);
    fn use_color(&self, color: ColorCode);
}

impl Shell {
    pub fn new(resolution: Resolution, font: Font, color: ColorCode) -> Self {
        Self {
            resolution,
            font,
            color,
        }
    }
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
