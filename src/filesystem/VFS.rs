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

// * FS drivers should implement these functions for the VFS
trait InodeFunctions {}

enum VNodeType {}

// the hierarchy depends on nefs-hierarchy.cfg
struct RootFS;

// represents a specific fs' superblock
struct VFSSuperBlock;

// remember to cache as much as possible in the vnode (in memory structure)
// so we dont have to make a request to disk for it or the inode cache
// basically everything in stat
struct VNode<'os> {
    // flags
    is_root: bool,
    // meta
    // when written to, CoW
    ref_count: u64,
    // reference to the mounted root. Could use a number instead
    mount_root: &'os VNode<'os>,
    // cache the type in the vnode itself
    vnode_type: VNodeType,
}

struct VFSFile;

/*
For reference, this is the stat struct of linux:

struct stat {
    dev_t     st_dev;          ID of device containing file
    ino_t     st_ino;          Inode number
    mode_t    st_mode;         File type and mode
    nlink_t   st_nlink;        Number of hard links
    uid_t     st_uid;          User ID of owner
    gid_t     st_gid;          Group ID of owner
    dev_t     st_rdev;         Device ID (if special file)
    off_t     st_size;         Total size, in bytes
    blksize_t st_blksize;      Block size for filesystem I/O
    blkcnt_t  st_blocks;       Number of 512B blocks allocated
    struct timespec st_atim;   Time of last access
    struct timespec st_mtim;   Time of last modification
    struct timespec st_ctim;   Time of last status change
}
*/
