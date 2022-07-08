---
layout: default
title: Services
parent: Neutron
---

## Neutron Services

I took inspiration from Zircon's design, where you have kernel objects that have a specific lifetime. A kernel object manages device io memory, processor timeslots, interrupts, IPC, and main memory.

## Rust API

Neutron syscalls are wrappers around asm `syscall/svc #0/scall` commands in x86/arm64/riscv. Syscalls are usually grouped into [6 major categories](https://en.wikipedia.org/wiki/System_call). But in Neutron, the syscall categories are simplified. And define a generic capability model for file management, device management and communication for higher level abstractions (languages) to develop policies for.

To use syscalls directly, link with `neutronapi`. Just using `std` is also fine and recommended for most use, unless your making a passthrough driver.

```rust
// no need to repr(C) since we arent using C at all, could still be useful for FFIs on other langs
#[repr(C)]
struct OpenFlags {
    read: bool,
    write: bool,
    execute: bool,
}

// type 1-N services have different numbers of arguments, from 1, 2, 3 ... N

// type A services dont have a return value
type ServiceResultA = ServiceResult<()>;

struct WaitOptions {
    status: WaitStatus,
    // -1 if not using
    pid: i64,
    signal: Signal,
}

// set or get
#[repr(C)]
union ProcessProperties {
    struct set_properties {},
    struct get_properties {}
}

enum SocketType {
    Stream,
    Datagram,
}

// can create a socket for the bottom 3 layers, IP, hardware, or just locally
enum SocketDomain {
    Localhost,
    Ipv4,
    Ipv6,
    Wifi,
    Bluetooth,
}

struct QuickTimeStamp {
    hours: u8,
    min: u8,
    secs: u8
}
```

| Service | Rust API | Notes |
| --- | --- | --- |
| **Object Management** |
| open | `fn open(filepath: &str, flags: OpenFlags) -> ServiceResult<FileDescriptor>` | Status = -1 on fail, e.g. no permissions. -2 on nonexistent filepath. flags -> RDONLY, RD_WRITE, APPEND, etc. |
| close | `fn close(fd: u64) -> ServiceResult<()>` | Status = -1 on fail if fd doesnt exist. -2 if trying to close a fd not owned by the user or an untrustworthy process |
| read | `fn read(fd: u64, nbytes: u64, buf: &str) -> ServiceResult` | Status = -1 if no more space. String can grow dynamically. -2 if trying to overread or do something fishy |
| write | `fn write(fd: u64, buf: &str, size: u64) -> ServiceResult` | Same idea as read for trying to overwrite the current size of buf |
| seek | `fn seek(fd: u64, offset: i64, type: SeekType) -> ServiceResult` | Seek an open file descriptor to a new offset, useful mostly for storage devices |
| chdir | `fn chdir(dest: &str) -> ServiceResult<()>` | |
| rmdir | `fn rmdir(dest: &str) -> ServiceResult<()>` | |
| rename | `fn rename(src: &str, dest: &str) -> ServiceResult<()>` | Rename any renamable file. Or moves it if the parent dir is different |
| stat | `fn stat(filepath: &str) -> ServiceResult<FileStats>` | |
| setstat | `fn setstat(filepath: &str, stats: FileStats) -> ServiceResult<()>` | Sets the attributes of the file. If it is executable, then also a level from 0-10 indicating how much the user/process trust that file. A low trustworthy file cannot run at all. All files downloaded from the internet should be 0 until the user explicitly gives a command to `trust` it, giving it a score of `7`. If it is from a verified publisher, then it has a score of `10`. If it cant run on Neutron, then its just another file |
| duplicate_fd | `fn duplicate_fd(int old_fd, int new_fd) -> ServiceResult<FileDescriptor>` | File descriptors are local to the process so it should be able to keep track of its open fd's and make new ones accordingly |
| **Process Control** |
| spawn | `fn spawn(filepath: &str) -> ServiceResult<ProcessID>` | Spawns a new process, as the child of the current one |
| exec | `fn exec(filepath: &str) -> ServiceResult<ProcessID>` | Replaces the current program with another one. All open fds are swept away and any volatile data too. The benefit is no process creation overhead |
| wait | `fn wait(wait_options: WaitOptions) -> ServiceResult` | Wait for a signal from a process. E.g. exit or kill |
| proc | `fn proc(properties: ProcessProperties) -> ServiceResult` | Set or Get a process' attributes |
| signal | `fn signal(signal: Signal, pid: ProcessID) -> ServiceResult` | Set or Get a process' attributes |
| mmap | `fn mmap(fd: FileDescriptor, protections: MemoryProtections) -> ServiceResult` | Request more memory (dynamic, stack memory is alloc'd automatically by sp). Or map a file/device functionality/memory into main memory |
| **~Device Management** |
| mount | `fn mount(src: &str, dest: &str) -> ServiceResult` | Mount a filesystem onto `dest`. The filesystem to be mounted should show somewhere in `/dev` or somewhere else on the vfs |
| dismount | `fn dismount(dir: &str) -> ServiceResult` | Dismount a filesystem by its dir |
| **~Communication** |
| socket | `fn socket(domain: SocketDomain, socket_type: SocketType) -> ServiceResult<FileDescriptor>` | Create a socket for this process |
| bind | `fn bind(socked_fd: FileDescriptor, address: SocketAddress) -> ServiceResult` | Bind one of this process's socket file descriptors from `socket` with a specified address |
| accept | `fn accept(socked_fd: FileDescriptor, address: SocketAddress) -> ServiceResult` | |
| connect | `fn connect(socked_fd: FileDescriptor, address: SocketAddress) -> ServiceResult` | Initiate a connection on a socket with a foreign address. Can take a while to return esp connecting remotely so should be called async |
| listen | `fn listen(socked_fd: FileDescriptor) -> ServiceResult` | |
| **System Configuration and Information** |
| sysinfo | `fn sysinfo() -> ServiceResult<SysInfo>` | Should be incorporated into __neutron_service_vdso |
| gettimeofday | `fn gettimeofday() -> ServiceResult<QuickTimeStamp>` | Get the time of day of the current locale. Should be incorporated into __neutron_service_vdso |
| setsysinfo | `fn setsysinfo(sysinfo: SysInfo) -> ServiceResult<()>` | Set system information. Requires root password |

## Notes

`spawn` is basically `fork` but more configurable. The child process still inherits the open fd tables and everything. It is CoW on default, although that can be turned off with CoW = false.

Some syscalls like `fork` and `terminate` shouldn't be needed. So I removed them.

In these syscalls, data is always moved in terms of ownership. There shouldnt be any copying because that wastes time. But maybe immutable borrow also works to ensure fastest performance. It just needs a lifetime `'a`.

For process control, some of the functions are supposed to be used by root processes like `init`. And some by High/Trusted processes to do things like exit. Exit can technically be used as `signal(EXIT, pid)` but we assume the current process doesnt know its pid. The syscall `signal(EXIT)` is only allowed for Root processes only and is checked by the kernel through `/sys/proc_permissions`. `signal` itself is not restricted. If a process tries to make `signal(KILL)` call without having the privileges for it, the kernel should intercept that and `pause()` that process. If `arcde` is on, it should also send a `signal(PERMISSION, pid=ArcRuntime)` which is running in Root Mode. Then it should render a window popup to prompt the user to give the process permissions to `resume()`.

The parent process should be able to enforce read only and etc. `__sparx_init` is running with root privileges. It should be able to set other process' rights to:

```rust
enum ProcessRights {
    Root,
    High
    Trusted,
    Untrusted
}
```

Most software should be running in High/Trusted mode. If some sparx detects something funny with a service, it can lower it to `Untrusted` to let `init` know to stop it from running or deprioritise it.

Processes running in `High` mode should not be able to do things like `rmdir` the entire root fs. The permissions of processes is stored in a privileged file in `/sys/proc_permissions`. If a process attempts to do something funny, it should just be stopped there and closed. Or with ArcDE, it shows a popup that asks the user to give the process permissions to do what it is trying to do. If granted, the process is allowed to keep going.

## Process Permissions

Process permissions are divided into 10 categories:

- File reads
- Modify files, change their location, delete them
- Access location info
- Access camera
- Access microphone
- Monitor input through /dev (basically read())
- Bluetooth
- Accessibility features

If done properly, users should be downloading verified apps. Which run in `High` mode. These apps should not do anything funny to the PC. And malicious sites should not be visited at all and all site browses should always be HTTPS enabled. Also other things like SAFE mode that prevents any popups and redirections at all. So that websites cant do stupid things.

Verified apps need to have a certificate and hash which can be checked. The hash is generated by the dev's 256-bit priv key with their software so it is practically infeasible to be an imposter.

## vDSO

Virtual Dynamic Shared Objects. An in memory data structure (basically an ELF .so file) that is linked to each program when they start running. Usually CoW and non-volatile so all programs should just be referencing those ~4 pages.

Can be used for a bunch of things like less problematic syscalls, which simply fetch some file or data elsewhere and return that in a uniform way. No need to figure out what /dev/... or /sys/... thing you need to search up and what operation to do on it to get some information. Basically provides a great standard API for all programs to use.

Like gettimeofday. Also in `/live/clock` with `read()` but why not just make the `gettimeofday` call to a vDSO function.
