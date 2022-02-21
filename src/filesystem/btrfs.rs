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

struct FileInfo;

// BTRFS FILES and DIRS
struct File {
    metadata: FileInfo,
}
// A file is contrasted to a directory since dirs have builtin children/pointers to `.` and `..`

struct Dir {
    metadata: FileInfo,
}

// Read: LBA, length, buffer
// Write: LBA, length, buffer

// given disk N, Partition P, use the underlying partition format functions to retrieve and edit the data
// from an SSD
pub trait FileOperations {
    // no async
    fn create_new(path: &str) -> Self;
    // no async
    fn delete();
    // async
    fn get_from_disk(disk_num: u64, block_address: u64, buffer: &str);
    // no async
    fn write_to_disk(disk_num: u64, block_address: u64, buffer: &str);
}

impl FileOperations for File {
    fn create_new(path: &str) -> Self {
        Self {
            metadata: FileInfo {},
        }
    }
    fn delete() {}
    fn get_from_disk(disk_num: u64, block_address: u64, buffer: &str) {}
    fn write_to_disk(disk_num: u64, block_address: u64, buffer: &str) {}
}

impl FileOperations for Dir {
    fn create_new(path: &str) -> Self {
        Self {
            metadata: FileInfo {},
        }
    }
    fn delete() {}
    fn get_from_disk(disk_num: u64, block_address: u64, buffer: &str) {}
    fn write_to_disk(disk_num: u64, block_address: u64, buffer: &str) {}
}

#[test]
fn test_files() {
    let _file = File::create_new("path");
}
