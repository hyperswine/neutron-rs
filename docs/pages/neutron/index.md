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

## Sparx

List:

- `spx:fs` => manages buffer cache and in memory filesystem view. Also logs events pushed to it by `spx:log` to disk. Since it keeps track of filesystem, its security is of concern. IDK I think a userspace/unprivileged service is fine, but it will also have to make syscalls and have userspace overhead
- `spx:system` => manages software commandline shell and is always pid 1, and the parent of all other userspace processes
- `spx:net` => manages network requests and buffers. Handles incoming packets and outgoing packets in a cache, where it will then publish the packets to listeners. Important as you dont want any random program to listen to your network activity
- `spx:input` => manages user generated IO interrupts. Can subscribe to keyboard events, mouse events, microphone events, and other input sensory events. App must be allowed certain permissions in order to subscribe to `input/kb`, `input/mouse`, `input/mic`, `input/webcam`, etc
- `spx:log` => logs diagnostic data to `/sys/log` every now and then. A pretty low priority service
- `spx:arc` => should be usually the among the first sparx to be initialised after other core system services. Manages the GUI shell and subscribes itself to `spx:input`, `spx:net`, `sop

Most events are pushed out as signals. So we use semaphore type semantics? Or more like signal-socket semantics, that are asynchronous in nature. So they can happen at any time, and your app should have a signal handler for that signal. Signals have high priority, and usually executes over what the current thread is doing. I think you can create a new user thread with a high priority for that process to execute the signal handler.

The file browser or `ls` could subscribe to `spx:fs`. And be implemented as mostly a `read()` call to the cwd. Then get that metadata and child files and write it to the console/stdout. I think println uses write which is a stream. So it will make an anon pipe and stream until EOF like usual. The scheduler or listener should pick up the new data and call the framebuffer driver `/dev/fb` to output its new generated frame. For hardware accelerated graphics, then it would be quite a bit different...

To create a new window, you need to use the window api. So you need to create a process, then subscribe to `spx:arc` and request a window of size and location. Then you are given a surface which you can render a texture to. If using graphics acceleration, you might use wgpu, which uses `spx:arc/graphics`. This will send all wgpu/vulkan requests to arc/graphics, which will then format and send the data to `/dev/<gpu>` that it is targeting, which will then process that into command buffers (usually already processed quite well) that can be DMA'd to the GPU or read directly (unified memory).

### Userspace vs Kernelspace services

Code running in kernel space should be mostly service handler calls that dictate mechanism, not policy. Otherwise the principle of least concern and modularity / data flow may be negatively impacted.

Some code like the fs service could prob be a kernelspace only service. Its code could be considered privileged code, and have the ability to directly call kernel handlers. But IDK, that might be harder to manage overall. Putting it in userspace as any other process but with flags like `spx`, and predetermined privileges for what its doing sounds like a good idea. So if you code it wrong, it wont break the system, it will just be terminated by the kernel itself or the shell supervisor.

The code in the kernel should be very secure and not doing too much. Attack surface and stuff too.
