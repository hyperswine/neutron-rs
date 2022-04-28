// ----------------------
// NEUTRON FS CENTRIC VFS
// ----------------------

// all filesystem drivers need to implement neutron vfs
// all fs related syscalls and processes operate on vfs

// FILE
struct File;

// -----
// DIR
// -----

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

// the hierarchy depends on nefs-hierarchy.cfg
struct RootFS;

struct Dir;

struct SymLink;

struct Device;
