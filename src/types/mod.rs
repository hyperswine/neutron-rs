pub mod bits;

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

// -------------------
// TIME
// -------------------

pub struct KTimestamp {
    day: u8,
    month: u8,
    year: u64,

    hour: f32,
    min: f32,
    sec: f32,
}

impl KTimestamp {
    // yyyy-mm-dd
    pub fn from_yyyy_mm_dd(str: &str) -> Option<Self> {
        // check if in right format
        if str.len() != 12 {
            return None;
        }

        let s = str.to_owned();
        let s = s.replace("-", "");
        // check first four are numbers 0-9
        let year = &s[0..3];
        let month = &s[4..5];
        let day = &s[6..7];

        let year = year.parse::<u64>();
        let year: u64 = match year {
            Ok(_) => year.unwrap(),
            Err(_) => return None,
        };

        let month = month.parse::<u8>();
        let month = match month {
            Ok(m) => {
                // check if m is between 1 and 12
                if m >= 1 && m <= 12 {
                    m
                } else {
                    return None;
                }
            }
            Err(_) => return None,
        };

        let month_31days = [1, 3, 5, 7, 8, 10, 12];

        let day = day.parse::<u8>();
        // depending on the month and year (leap year), get the max date
        let day_max = match day {
            Ok(d) => {
                // if january, march, etc. always 31 days
                if month_31days.contains(&month) {
                    31 as u8
                }
                // if feb, check if leap year
                else if month == 2 {
                    if (year % 400 == 0 && year % 100 == 0) || (year % 4 == 0 && year % 100 != 0) {
                        29
                    } else {
                        28
                    }
                }
                // if june, nov, etc
                else {
                    30
                }
            }
            Err(_) => return None,
        };

        let day = day.unwrap();
        if day > day_max {
            return None;
        }

        Some(Self {
            day,
            month,
            year,
            hour: 0.0,
            min: 0.0,
            sec: 0.0,
        })
    }
}
