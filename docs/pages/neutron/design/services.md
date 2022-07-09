---
layout: default
title: Services
parent: Neutron
---

## Neutron Services

I took inspiration from Zircon's design, where you have kernel objects that have a specific lifetime. A kernel object manages device io memory, processor timeslots, interrupts, IPC, and main memory.

With syscalls, data is always moved in terms of ownership. There shouldnt be any copying because that wastes time. Immutable borrow also works if there are no mutable borrows, though for more complex concepts, mutable ARCs/RefCells are needed, as well as lifetime specifiers `'service` or `'ns`.

### On UNIX Syscalls vs Neutron

`spawn` is basically `fork` but more configurable. The child process still inherits the open fd tables and everything. It is CoW on default, although that can be turned off with CoW = false. `fork` and `terminate` shouldn't be needed. So I removed them.

### Process Control

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

## vDSO

Virtual Dynamic Shared Objects. An in memory data structure (basically an ELF .so file) that is linked to each program when they start running. Usually CoW and non-volatile so all programs should just be referencing those ~4 pages.

Can be used for a bunch of things like less problematic syscalls, which simply fetch some file or data elsewhere and return that in a uniform way. No need to figure out what /dev/... or /sys/... thing you need to search up and what operation to do on it to get some information. Basically provides a great standard API for all programs to use.

Like gettimeofday. Also in `/live/clock` with `read()` but why not just make the `gettimeofday` call to a vDSO function.
