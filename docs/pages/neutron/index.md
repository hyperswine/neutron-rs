---
layout: default
title: Neutron
has_children: true
---

## Key Things

Neutron is a minimalist data driven kernel. As a program, its main purpose is to arbitrate the needs of processes that wish to do certain things and the user/environment, who want to make the system do certain things.

- [Services](services.md)
- [Rust Std](rust-std.md)

### Movement of Data

The flow of data is the key thing of interest in Neutron. How do we make everything flow seamlessly and efficiently. I bring you the highway analogy. The highway has many entry points and exit points, but is essentially a long strip of road (a "bus") where traffic flows. Some traffic wants to enter at specific points and some want to exit at other points. Some need to stay on the highway longer to get to its destination, some take the exit almost immediately as they enter. Each entry/exit point is basically a switch or interchange where traffic tends to build up.

Data needs to be queued if the bus is already being used to and from the destination. This is one of the primary causes of traffic. So gotta make the best use of the hardware and have hueristics to move data around. Theres priority lanes for certain types of data. 3 Levels, high, standard, low. If a packet is of high prio, it should be moved into the prio queue with a high prio. Low prio traffic can simply be appended to the back of the queue. Normal prio can overtake low prio data but can also be overtaken by high prio data.

## Apps

A `no-std` app can run on Neutron as long as it is compiled to the right format. Neutron does not have any hosted tools to compile for itself so one must cross compile for now. To do more useful things, [`neutronapi`](neutronapi.md) can be used, though existing apps that make use of `std` benefit from compiling to `riscv64gc-neutron-elf` or `aarch64-neutron-elf`.

## Kernelspace - Everything is an Object

I quite like the generic idea of Fuchsia (Zircon kernel). Where a lot of your syscalls are quite generic and have more to do with the movement of data rather than any specific concept like devices, file management, etc.

In Zircon you basically replace the concept of Files with the concept of Objects. Objects are more generic than files and both have handles to faciliate their lifecycle and management. Its mostly the naming and formalisation that has changed.

## Userspace - Everything exists in the Filesystem

In userspace, it is actually not a bad idea to think of most things as just files. They are easy to manage, create and destroy and expose the same interface. The file browser or terminal `ls` can give a very fast and simple view of the system resources, both hard and soft.

The main filesystem is [NeutronFS](neutronfs/index.md). The VFS is pretty much NeFS in-memory mode. NeFS is the on-disk representation. Other types of FS are supported through mounting its partition as a VFS file. And can be mounted automatically with an NeFS boot mount record.

Most apps and background tasks (sparx) run in usermode. HTTP servers and stuff will utilise user kernel networkd sockets exposed in `/dev`. Apps like umbral word will use filesystemd through file based mmio ipc or syscalls.

## Installer

The installer for neutron-quantii comes in 2 parts, one in the browser and one on the actual system.

### Part 1

Phase 1 of Neutron Installation takes place on the browser where a user uses a tool to flash a live image of arcboot + neutron onto a removable drive.

This uses the webusb standard and any wasm processes that does operations like `dd` or basically what balenaEtcher does.

Here, large scale activities like choosing specific kernel modules to add can be done. If a user doesnt need wasm support, they dont need a kernel image with it loaded. If they want it later, they can do so in software by linking the module into an existing/new image and rebooting into it.

### Part 2

Phase 2 of Neutron Installation takes place locally on a supported machine for the flashed target. E.g. an aarch64 machine.

Secure boot and other advanced BIOS features should be disabled. UEFI should be enabled. The system should be able to locate and boot Arcboot via `/boot/arcboot.efi`.

From there, Arcboot loads the neutron kernel and a default userspace configuration (ArcConfig) through `/sys/config` files. Only single user root boot is supported. Multi users must be setup in software.

If `/sys/config/new_user` is there (which it should be on a new install), Arc should load the minimal WM + DE config with the installer. The user is expected to go through installation of the files onto a permanent disk partition before being able to use the system. In this step, `/sys/config` is modified and files are copied from the live stick to the system drive.
