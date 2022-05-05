// ----------------------
// NEUTRON VFS
// ----------------------

// all filesystem drivers need to implement neutron vfs
// all fs related syscalls and processes operate on vfs
// filesystem drivers should try to implement this as best as possible

/*
nefs-hierarchy.cfg

# REQUIRED
/sys
    /dev
        tty
        null
    /vars
        path
        local.pid_<pid>
    /config
        boot.cfg # basically .env format. Sets kernel modules to be loaded
        init.cfg # sets daemons/services/sparx to be loaded as a list of paths to those executables. Prob in /sys/bin
    init.elf
    /bin
        /network
            __sparx_wifi
            __sparx_eth
            __sparx_bt
            __sparx_dhcp
            /server
                __sparx_http
                __sparx_ftp
                __sparx_grpc
        /io
            __sparx_filesystem # rootfs service for managing reads and write queues and caches. Kinda like syncd
            __sparx_devices # management of devices through managing queues and caches for /sys/dev
        /log
            __sparx_syslog
        /time
            __sparx_network_time
        /cron
            __sparx_cron
        /system
            __sparx_system_init # the main init daemon that spawns all other processes like the cli shell or DE. The DE then becomes the defacto spawner of GUI apps. But make no mistake, its still a child of system_init

/home # DE + workspaces
/dev
/mnt

# RECOMMENDED
/packages
/snapshots
*/

// AFTER THE ROOTFS is mounted:
// read() /sys/config/disk.load
// this is similar to fstab
// load disk partitions from /dev/nvme0p2, etc into /mnt/<name_of_partition>
// extra devices can also be loaded into /dev
// such as any virtual devices or virtual fs

// ----------------------
// TYPES
// ----------------------

// * open(), write(), etc. services should use these primitives

pub enum VNodeType {
    RegularFile,
    Dir,
    Device,
    Fifo,
    Socket,
    Symlink,
}

// the hierarchy depends on nefs-hierarchy.cfg
pub struct RootFS;

// represents a specific fs' superblock
pub struct VFSSuperBlock;

// remember to cache as much as possible in the vnode (in memory structure)
// so we dont have to make a request to disk for it or the inode cache
// basically everything in stat

// A pointer to an inode in a filesystem or device
pub struct VNode {
    // flags
    is_root: bool,
    // meta
    // when written to, CoW
    ref_count: usize,
    // reference to the mounted root. Could use a number instead
    mount_root: u64,
    // cache the type in the vnode itself
    vnode_type: VNodeType,
}

const LOGICAL_BLOCK_SIZE: usize = 4096;

// all filesystem drivers should implement this
pub struct FileStat {
    // should only be kept in vnodes. VFS supports up to 2^64 devices
    // if it is a special file like a gpu, it should be that instead
    dev_id: u64,
    inode_id: u64,
    n_hard_links: usize,
    total_size_bytes: usize,
    // generally 4K. Though depends on the filesystem. If a char device, then it is 1 since we are always streaming data to/from it serially
    io_block_size: usize,
    // in multiples of logical block size, always multiples of 4K (LOGICAL_BLOCK_SIZE)
    n_blocks_allocated: usize,
}

// Files should be opened by processes in read mode only
// And only granted write permissions when the user accepts the prompt

// ----------------------
// OPEN FILE TABLE
// ----------------------

// note, paging and heap needs to be initialised before the open file table
// do so in `kernel_entry`

use alloc::vec::Vec;

// Main table in memory that stores a reference to each file descriptor -> vnode
// Files that have a ref count of 0 may be cleaned up or left there in case the user wants to reopen it
struct OpenFileTable<'os> {
    open_vnodes: Vec<&'os VNode>,
}
