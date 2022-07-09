---
layout: default
title: Neutron API
parent: Neutron
---

## Kernel Overview

When writing kernel level drivers or kernel modules, one should use `neutron_api`.

## NeutronAPI Userspace

Most things can be built with `rust-std` or bare neutron service suite with rei. However apps that are graphical in nature should use `arckit` which bundles `arcwm`, general wrapper classes and an object persistence library for storing and manipulating data.

When building executables/libraries with rust for `target_os=neutron_arc64`:

```rust
// in neutronapi::

struct ServiceResult<T> {
    data: Option<T>,
    status_code,
    // should actually make this a 'some_lifetime &str
    // and give ownership to the return function
    message: Option<String>,
}

// wrappers around asm calls and more checks (that arent in the kernel cause we want to keep it lean)
fn sys_read(arg1, arg2) -> ServiceResult {
    // checks
    if arg1 is uninitialised or out of scope {
        return fail
    }

    if arg1 + arg2 is uninitialised or out of scope {
        return fail 
    }

    unsafe {
        // make the syscall
        asm!("...");

        // asm should load the return val in r0, load it into a variable
        let result = u64;
        asm!("ld {result} r0");
    }

    ServiceResult {
        result,
        SUCCESS,
        None,
    }
}

```

## Rust Syscall API

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
