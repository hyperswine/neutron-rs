---
layout: default
title: Rei API
parent: Rei
---

## Rei API for Neutron

A generalised service interface using `extern rei`:

| Service | Rei API | Notes |
| --- | --- | --- |
| mount | `fn mount(dev_id: u64, filepath: &str) -> ServiceStatus` | Can be used on almost any device with fd semantics |
| dismount | `fn dismount(filepath: &String) -> ServiceStatus` | |
| open | `fn open(filepath: String, flags: u8) -> (ServiceStatus, <fd>)` | Status = -1 on fail, e.g. no permissions. -2 on nonexistent filepath. flags -> RDONLY, RD_WRITE, APPEND, etc. |
| close | `fn close(fd: u64) -> (ServiceStatus)` | Status = -1 on fail if fd doesnt exist. -2 if trying to close a fd not owned by the user or an untrustworthy process |
| read | `fn read(fd: u64, nbytes: u64, buf: &String) -> ServiceStatus` | Status = -1 if no more space. String can grow dynamically. -2 if trying to overread or do something fishy |
| write | `fn write(fd: u64, buf: &String, size: u64) -> ServiceStatus` | Same idea as read for trying to overwrite the current size of buf |
| lseek | `fn lseek(fd: u64, offset: i64, type: SeekType) -> ServiceStatus` | Seek an open file descriptor to a new offset, useful mostly for storage devices |
| stat | `fn stat(filepath: &String) -> (ServiceStatus, <FileStat>)` | |
| dup | `fn dup(fd: u64) -> (ServiceStatus, <new_fd>)` | Duplicate an fd. Useful for two processes/threads reading/writing to the same file or resource |
| socket | `fn socket() -> (ServiceStatus, <socket_fd>)` | Represents a endpoint for a channel of communication, usually between a server-client or just two peers |
| bindsocket | `fn bindsocket(socket_fd, addr: SocketAddr) -> ServiceStatus` | Bind a name to the socket |
| send | `fn send(socket_fd, buf: &String, flags: SendFlags) -> ServiceStatus` | Send an ASCII message stored in a userspace buffer to the address connected to by the socket. Needs either socket_bind or connect beforehand |
| connect | | Attempt to connect to an address, usually ipv4/6. Can be localhost:8000 for example |
| accept | | Accept an incoming request. Usually used in a loop with a queue of requests |
| spawn | `fn spawn(executable_path: &String, args: &Vec<String>) -> (ServiceStatus, <pid>)` | Spawn a new process by creating a userspace container environment based on the executable's trustworthy level. execute it (elf64 only) and give it a priority of `PriorityDefault` |
| nice | `fn nice(pid: u64, nice_level: i64) -> ServiceStatus` | Should be used by the user only. Kernel management should be relied on in most cases |
| time | `fn time() -> (ServiceStatus, <u64>)` | Returns the number of seconds elapsed since 1970-01-01 00:00 |
| gettimeofday | `fn gettimeofday() -> (ServiceStatus, <Timestamp>)` | Should be used with a high level construct like rust-std, which should use the VDSO version rather than the syscall |
| symlink | `fn symlink(old_path: &String, new_path: &String) -> ServiceStatus` | |
| chmod | `fn chmod(path: &String, flags: ChownFlags) -> ServiceStatus` | |
| chown | `fn chown(path: &String, u_id: u64) -> ServiceStatus` | |
| chdir | `fn chdir(new_dir_path: &String) -> ServiceStatus` | |
| mmap | `fn mmap(addr: *data, length: u64, flags: MMapFlags) -> ServiceStatus` | Maps a file to RAM (the process's vm). Works for a device using its `dev_id` too. If you want to `open` to read/write efficiently, can use mmap. Kernel handles writing back changes at idle time |
| heapalloc | `fn heapalloc(new_dir_path: &String) -> ServiceStatus` | Basically brk() and sbrk() combined for anything malloc() related. For stack based data, the program should be able to as much as it wants directly `<= stack_limit` which is usually quite high or `unlimited` in virtual memory  |
| ulimit | `fn ulimit() -> (ServiceStatus, ULimit)` | Often used to see the usage limits like RAM/stack space available to the current user/session |

### Notes

- some less contentious and safer services can be implemented as a VDSO
- `ServiceStatus` combines the generic return value of linux syscalls with the more helpful errno. Also includes a message in `ServiceStatus.message`
- the fd's are actually wrapped around Option<> but Im shortcutting them as `<type>` instead of `Option<Type>`
