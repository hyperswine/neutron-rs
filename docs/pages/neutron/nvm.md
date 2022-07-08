---
layout: default
title: Nevermind
parent: Neutron
---

## Forget about it

So I have a bunch of things that I dont really care about anymore. I'll just paste them here.

## List of Services

Although there is no need to think of everything as a file, the concept of a file being openable, closable, readable, writable is quite good for many things

- For most devices, we can mount(dev_id) as a file and then open(filename) to initialise it
- Then we can `read` and `write` like usual, e.g. to a wifi card (socket), disk, speaker, mic, display, gpu

For the most part, syscalls are there to control access for processes, not users. The user is assumed to be the root. And some level of protection against hacking / remote or physical hijacking as well. But those can be done with system wide locks like `sleep`. Remote scripts and hacks can change their dirs and stuff, which might not be a great thing so we should make some parts of the fs restricted to root (needs password prompt) and or privileged process / super privileged boot.

### API

Shells and apps that want to use neutron functions directly should install the `neutron_api` lib and call its functions according to the type of file one is dealing with. Otherwise it is perfectly fine to use `std` implementations for userspace programs.

```rust
// in neutronapi::vfs

enum StatusCode {
    Success,
    Fail,
}

// as long as the service call remains active
// should be moved
struct Status<'service> {
    code: StatusCode,
    message: &str
}

fn register_fs(fs: &Fs) -> Status;
fn deregister_fs(fs: &Fs) -> Status;

enum ConnectionMethod {
    OneWay,
    TwoWay,
    MultiWay,
}

// open a connection to a list of processes
fn open_connection(ConnectionMethod, processes: &[Process]) -> Status;
```

## Everything is a Concept

A concept is an idea. Which can encompass other sub ideas. A concept exists in logical space and must be implemented in physical space. Together, we have a hierarchy of ideas from the most general to the least.

In Neutron, everything is a concept. Physical layers are built ontop of logical layers. Logical layers which are built ontop of logical layers.

This way we can easily modularise each part of the kernel's implementation and allow abstractions to flow naturally.

### Concept: Filesystem

A filesystem is a place where "files" are stored. Files are generic enough to be implemented however fit.

The VFS implements all supported filesystems in its physical space.
