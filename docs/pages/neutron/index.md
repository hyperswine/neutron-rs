---
layout: default
title: Neutron
has_children: true
---

## Key Things

- [Memory Management](memory.md)
- [Filesystems](vfs.md)
- [Services](services.md)
- [Rust Std](rust-std.md)

The main filesystem is [NeutronFS](neutronfs/index.md). The VFS tries to stay close to it as much as possible while being generic enough to support multiple different filesystems seamlessly (via mount).

A `no-std` app can run on Neutron as long as it is compiled to the right format. Neutron does not have any hosted tools to compile for itself so one must cross compile for now. To do more useful things, [`neutronapi`](neutronapi.md) can be used, though existing apps that make use of `std` benefit from compiling to `riscv64gc-neutron-elf` or `aarch64-neutron-elf`.

## Questions

Should more things be in the userspace, or the kernel space? I think generally userspace. Or even the firmware if possible.

A filesystem does not have to reside in kernel space. But from the principle of least responsibility and abstraction building. I think it kinda makes sense.

As long as everything is in one language and is easy to build. Which is doable with rust modules and cargo. That means it doesnt really matter where you write a specific component. Just that it does exist. And isnt in the way of other things.

Basically a case by case basis? Maybe not all filesystem drivers have to be in the kernel or userspace. Some can be in the kernel. Some in userspace. If the code is generally lean and clean. As well as generic enough to support extensions. And easy enough to refactor to be more generic or better. I dont see a big problem.

## Everything is a Concept

A concept is an idea. Which can encompass other sub ideas. A concept exists in logical space and must be implemented in physical space. Together, we have a hierarchy of ideas from the most general to the least.

In Neutron, everything is a concept. Physical layers are built ontop of logical layers. Logical layers which are built ontop of logical layers.

This way we can easily modularise each part of the kernel's implementation and allow abstractions to flow naturally.

### Concept: Filesystem

A filesystem is a place where "files" are stored. Files are generic enough to be implemented however fit.

The VFS implements all supported filesystems in its physical space.

### Concept: Genericism

I quite like the generic idea of Fuchsia (Zircon kernel). Where a lot of your syscalls are quite generic and have more to do with the transport of data rather than any specific concept like process management, file management, etc.

Though I guess with Zircon you can basically replace the concept of Files with the concept of Objects. Objects sound more generic than files. But in a way, it still quite similar anyway.

## Userspace - Everything exists in the Filesystem

In userspace, it is actually not a bad idea to think of most things as just files. They are easy to manage, create and destroy and expose the same interface.

Your apps and most background tasks (sparx) run in usermode. HTTP servers and stuff will utilise user kernel networkd sockets exposed in `/dev`. Apps like umbral word will use filesystemd through file based mmio ipc or syscalls.

## VFS

Any system IO requests and etc. should use the kernel's VFS. It wraps around all supported filesystems with driver implementations.

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

## Neutron Installer P1

Phase 1 of Neutron Installation takes place on the browser where a user uses a tool to flash a live image of arcboot + neutron onto a removable drive.

This uses the webusb standard and any wasm processes that does operations like `dd` or basically what balenaEtcher does.

Here, large scale activities like choosing specific kernel modules to add can be done. If a user doesnt need wasm support, they dont need a kernel image with it loaded. If they want it later, they can do so in software by linking the module into an existing/new image and rebooting into it.

## Neutron Installer P2

Phase 2 of Neutron Installation takes place locally on a supported machine for the flashed target. E.g. an aarch64 machine.

Secure boot and other advanced BIOS features should be disabled. UEFI should be enabled. The system should be able to locate and boot Arcboot via `/boot/arcboot.efi`.

From there, Arcboot loads the neutron kernel and a default userspace configuration (ArcConfig) through `/sys/config` files. Only single user root boot is supported. Multi users must be setup in software.

If `/sys/config/new_user` is there (which it should be on a new install), Arc should load the minimal WM + DE config with the installer. The user is expected to go through installation of the files onto a permanent disk partition before being able to use the system. In this step, `/sys/config` is modified and files are copied from the live stick to the system drive.
