// ------------------
// NEUTRON FILESYSTEM
// ------------------

// like btrfs but pretty cool (not unstable)
// you can also just use guest containers with a simpler semantic fs for a multi user setup or vm

// get MMIO addresses from ACPI or device tree for a specific partition
// 

struct MMIO_API {
    // address LBA of disk
    write_LBA_addr: u64,
    read_LBA_addr: u64,
}

impl MMIO_API {
    fn new(write_LBA_addr: u64, read_LBA_addr: u64) -> Self { Self { write_LBA_addr, read_LBA_addr } }
}

// TODO: query kernel device tree (not firmware) for partition num
// and extra details
fn get_mmio_api_from_partition() -> MMIO_API {
    MMIO_API::new(0, 0)
}

// ------------------
// PARTITION METADATA
// ------------------

// inode tables, etc.
struct Superblock {}

use alloc::collections::btree_map::BTreeMap;

use crate::types::KTimestamp;

// there can be 1.8 quintillion users 
type NeutronUUID = u64;

const MAX_FILE_SIZE_BYTES: u64 = 1024_u64.pow(6);
// TODO: technically, the sector size should be 4KiB. But the 'Node' size should be 16KiB
const BLOCK_SIZE_BYTES: usize = 4192;

// ------------------
// INodes
// ------------------

// a step below vnodes

type FilePermissions = u16;

#[repr(C, packed)]
struct INode {
    // ------USER INFO------
    creator_id: NeutronUUID,
    owner_id: NeutronUUID,
    // d-rwx-rwx-rwx (10 bits, 6 bits empty)
    permissions: FilePermissions,
    // ------FLAGS------
    rd_only: bool,
    hidden: bool,
    system_use: bool,
    // if 1, then back it up
    needs_to_be_backed_up: bool,
    // ascii or binary (including utf 8)
    is_ascii: bool,
    // 0 if sequential access only, 1 if not
    allow_random_access: bool,
    // for filesystemd to figure out
    locked: bool,
    // ------KEY INFO------
    // number of bytes in a record
    bytes_per_record: u64,
    offset_of_key: u64,
    key_length: u64,
    // ------TIME INFO------
    creation_time: KTimestamp,
    last_accessed: KTimestamp,
    last_changed: KTimestamp,
    // ------BLOCK INFO------
    // in bytes, content only, not inode/blocks
    curr_size: u64,
    // should always be MAX_FILE_SIZE_BYTES (16 EiB)
    max_size: u64,
    block_btree: BTreeMap<u64, Block>,
}

impl INode {
    fn new(
        creator_id: NeutronUUID,
        owner_id: NeutronUUID,
        permissions: FilePermissions,
        rd_only: bool,
        hidden: bool,
        system_use: bool,
        needs_to_be_backed_up: bool,
        is_ascii: bool,
        allow_random_access: bool,
        locked: bool,
        bytes_per_record: u64,
        offset_of_key: u64,
        key_length: u64,
        creation_time: KTimestamp,
        last_accessed: KTimestamp,
        last_changed: KTimestamp,
        curr_size: u64,
        max_size: u64,
        block_btree: BTreeMap<u64, Block>,
    ) -> Self {
        Self {
            creator_id,
            owner_id,
            permissions,
            rd_only,
            hidden,
            system_use,
            needs_to_be_backed_up,
            is_ascii,
            allow_random_access,
            locked,
            bytes_per_record,
            offset_of_key,
            key_length,
            creation_time,
            last_accessed,
            last_changed,
            curr_size,
            max_size: MAX_FILE_SIZE_BYTES,
            block_btree,
        }
    }
}

struct Block {}
