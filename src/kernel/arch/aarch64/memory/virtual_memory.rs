// ----------------
// PAGING
// ----------------

// needs to be turned on in order to use heap allocation
// on 4K pages

use aarch64::paging;

// dynamically allocate all entries
// per level
const N_LEVELS: u8 = 4;

// should be alloc'd before entry into the kernel
// an entry can point to another entry in the same table or a different table
// up to 5 indirections for 52 bits
struct PageTable<'prior> {
    entries: &'prior mut u64,
}

// 52-bit vaddr
// make sure to use cortex > a-55 / > a75
#[repr(C)]
struct PageTableEntry {
    // 0-15
    offset: u16,
    l3_index: [bool; 13],
    l2_index: [bool; 13],
    l1_index: [bool; 10],
    base_addr_of_translation_table_1: [bool; 13],
}

// CANT USE
// because it relies on paging
// use alloc::vec::Vec;
