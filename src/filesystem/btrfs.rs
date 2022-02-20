// Based on https://btrfs.wiki.kernel.org/index.php/Btrfs_design

#[cfg(not(test))]
use alloc::{boxed::Box, rc::Rc, string::String, string::ToString, vec, vec::Vec};

// ---------------
// BTRFS B-TREE
// ---------------

#[repr(C)]
struct BtrfsHeader {
    csum: [u8; 32],
    fsid: [u8; 16],
    bytenr: u64,
    flags: u64,

    chunk_tree_uid: [u8; 16],
    generation: u64,
    owner: u64,
    nritems: u32,
    level: u8,
}

#[repr(C)]
struct BtrfsDiskKey {
    object_id: u64,
    _type: u8,
    offset: u64,
}

#[repr(C)]
struct BtrfsItem {
    key: BtrfsDiskKey,
    offset: u32,
    size: u32,
}

// Directories/Files are BtrfsItems and each item has an associated key
// The BtrfsHeader specifies a new btrfs block, based on a hierarchical view. E.g. a partition, virtual partition, top level directory, mounter partition

// ------------------
// Filesystem Viewer
// ------------------

struct File;
// A file is contrasted to a directory since dirs have builtin children/pointers to `.` and `..`

#[test]
fn test_files() {
    let _file = File {};
}
