// ------------------
// NEUTRON FILESYSTEM
// ------------------

// like btrfs but pretty cool (not unstable)
// you can also just use guest containers with a simpler semantic fs for a multi user setup or vm

// get MMIO addresses from ACPI or device tree for a specific partition

struct MMIO_API {
    // address LBA of disk
    write_LBA_addr: u64,
    read_LBA_addr: u64,
}

impl MMIO_API {
    fn new(write_LBA_addr: u64, read_LBA_addr: u64) -> Self {
        Self {
            write_LBA_addr,
            read_LBA_addr,
        }
    }
}

// TODO: query kernel device tree (not firmware) for partition num
// and extra details
fn get_mmio_api_from_partition() -> MMIO_API {
    MMIO_API::new(0, 0)
}

// ------------------
// PARTITION METADATA
// ------------------

type FSUUID = [u8; 16];
// CRC32
type Checksum256 = [u8; 256];

// might have to be the same size as btrfs superblock for checksums to work properly
// should be exactly 65536 Bytes. So 256 256-blocks for CRC32C or SHA-2 (slower)
#[repr(C)]
struct Superblock {
    checksum: Checksum256,
    fs_uuid: FSUUID,
    // on disk LBA of the start of this block
    physical_addr: u64,
    flags: u64,
    // technically just 8 ASCII bytes, should be "__NeFS__"
    // unless its an extension or modified version of NeFS
    magic: u64,
    generation: u64,

    // ROOT POINTERS
    core_tree_root_logical_addr: u64,
    chunk_tree_root_logical_addr: u64,
    log_tree_root_logical_addr: u64,
    // transaction id for log root
    log_root_transaction_id: u64,

    // SIZE
    // size of partition
    total_bytes: u64,
    // size of used blocks, including superblocks and redundancies
    bytes_used: u64,
    // usually 6 for the root filesystem
    root_dir_object_id: u64,
    // at least one. Could be 2^64 for RAID
    number_of_devices: u64,
    sector_size: u32,
    node_size: u32,
    leaf_size: u32,
    stripe_size: u32,
    // size of a single chunk (array of chunks)
    system_chunk_array_size: u32,

    // OTHER
    chunk_root_generation: u64,
    compatibility_flags: u64,
    compatibility_read_only_flags: u64,
    incompatibility_flags: u64,
    // should be CRC32C
    checksum_type: u16,
    root_level: u8,
    chunk_root_level: u8,
    dev_item: [u8; 0x62],
    // label for the partition
    label: [u8; 0x100],
    cache_generation: u64,
    uuid_tree_generation: u64,
    reserved: [u8; 0xF0],
    sys_chunk_array: [u8; 0x800],
    super_roots: [u8; 0x2A0],
    unused: [u8; 0x235],
}

use alloc::collections::btree_map::BTreeMap;

use crate::types::KTimestamp;

// there can be 1.8 quintillion users
type NeutronUUID = u64;

const MAX_FILE_SIZE_BYTES: u64 = 1024_u64.pow(6);
// TODO: technically, the sector size should be 4KiB. But the 'Node' size should be 16KiB
const BLOCK_SIZE_BYTES: usize = 4192;

// ------------------
// Nodes
// ------------------

type FilePermissions = u16;

struct NeutronFSNodeHeader {
    checksum: Checksum256,
    fs_uuid: FSUUID,
    logical_address: u64,
    flags: [u8; 7],
    // should be 1 for new filesystems otherwise 0 for an old filesystem
    back_reference: u8,
    chunk_tree_uuid: FSUUID,
    // ? the generation of the header
    generation: u64,
    id_of_parent: u64,
    number_of_child_items: u32,
    // 0 = leaf, I think also includes core root nodes
    level: u8,
}

// yyyy-mm--ddThh:mm:ss + nanosecs
// prob not that accurate anyway due to hardware latency
struct UnixTime {
    seeconds_since_epoch: u64,
    nanoseconds: u32,
}

struct NeutronFSKey {
    object_id: u64,
    item_type: u8,
    // meaning of this actually depends on the type
    offset: u64,
}

struct InternalNode {
    key: NeutronFSKey,
    block_num: u64,
    generation: u64,
}

struct LeafNode {
    key: NeutronFSKey,
    // relative to the end of the header. So where the first payload starts for the data section
    data_offset: u32,
    data_size: u32,
}

// TODO
enum DType {}

struct NeutronFSItem {
    // the key of the inode_item or root_item associated with this item
    // prob the key of the parent dir
    key: NeutronFSKey,
    offset: u32,
    size: u32,
}

// Actual NeutronFS item
// Can either be another dir, regular file, device file, link
struct NeutronFSDirItem {
    location: NeutronFSKey,
    transaction_id: u64,
    // length of the extended attributes associated with this item. Just 0 for a dir. For a file or something else, might be 0-16k
    data_length: u16,
    // name of the directory entry (not the file)
    name_length: u16,
    d_type: DType,
}

// Supposed to be indexed through (inode_number, inode_item, parent_inode)
// to find the DirItem entires/filename for a given inode
struct NeutronFSInodeRef {
    index: u64,
    name_length: u16,
}

// A type of Node that describes a file
// https://btrfs.wiki.kernel.org/index.php/Data_Structures#btrfs_inode_item
#[repr(C, packed)]
struct NeutronFSINode {
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
}

// just needs to contain the stat struct https://linux.die.net/man/2/stat
impl NeutronFSINode {
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
        }
    }
}

// Logical block / payload of a leaf node
struct Block {}
