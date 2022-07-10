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

### Subscribing to Services

When you subscribe to a service, you also have to specify the types of signals you want to subscribe to. For keyboard events, you would subscribe to `spx:arc` if using windowing/ArcDE. Or maybe Arc takes those spx as its own children.

You subscribe to a service by making the right syscalls. Normally you would use neutronapi that contains the right order of calls in order to subscribe. The API also defines the abstracted structs that you pass in and receive. Processes may have a `key` that can be checked against to verify that they have access. This is all stored in `spx:arc` or `spx:system` memory.

Normally, you would make a syscall to create a channel between your process and the service process. I dont think you would use a pipe. A socket could also work. And you write bytes to the channel or socket stream followed by EOF as usual. That should be a syscall like `write()` which would do a zero-copy operation, and share those new pages (virtual memory object) with the other process. It may then signal that other process or not, prob not. Services are usually idle loops that keep checking whether it has new data. So the next time the service is scheduled, it should have the new data. If that data would have to be changed during or after the write, it would use CoW semantics to create a new VMO, and point to it after its done. That would be done in the kernel write() handler if it detects an existing VMO is being written to.

Services essentially are `loop { if new_data { do_something(new_data); } }`. And usually have low priority. But once you try to communicate with the service, the kernel may bump the priority of it up by a certain amount. The scheduler would then see the new priority value in its list of kernel threads waiting to execute.

### Scheduler

The scheduler itself is mostly a circular priority queue. It is a kernel subsystem with no userspace component. So theres not much space for a malicious process to do something stupid. Now older entries would be pushed out and lost. But theres not much you can do about it. Its quite efficient though and resistant against memory corruption.

As a queue, it queues new kernel threads in O(1) time and outputs in O(1) time for the most part. It achieves this through randomisation hueristics. By assigning each thread `priority, ticks_passed` index. So the space it takes is around `N * u64 * u64`. The ticks passed is incremented each time the scheduler is automatically invoked by the clock interrupt. The clock interrupt is handled by one of the cores, and supersedes most interrupts by priority so a syscall or hardware handler be interrupted. The scheduler is mostly a single threaded process for simplicity and the fact that you are generating 8 random values and deciding which thread should go next on which core, could be harder to multithread.

Once the scheduler decides, it will push the kthread ids to their respective cpu queue including its own, and return to the next thread waiting on its own cpu's queue.

Each CPU has its own kthread queue. Each entry is simply a pointer to the actual kthread somewhere in kernelspace. The kthread contains a metadata about the register values and a pointer to the `.text` segment or `code` object that the cpu can jump to and start executing. Maybe just store the address (vaddr) of the instruction to start executing from.

The problem is when you start relocating things. The CPU should deal with vaddr for the most part. The changes should mostly be in one place where things that reference it have access to. Like a page table. Relocating the page should be mostly automatic if your using vaddr.

```python
def scheduler_tick():
    random_vals = random.vals(8)
    for i,cpu in enumerate(cpus) {
        cpu.queue(kthread[random_val[i]])
    }

    return cpu.this_thread.first
```

### spx:system

Kind of like systemd, and contains something similar to udev, or maybe hands that off to `spx:dev`. It should maybe contain something like `dbus`. I think that would be nice. So you have a unified way of creating services and bidirectional communication. And you just have to ask your ancestor for it. If `arc` is enabled, it would then hand off most requests to it, which should then render as a dialog option asking the userr if they wish to grant the asking app the permissions it wants to communicate with other privileged processes or use system resources.

Maybe when `arc` is started, it just disables its own handler and enables it to listen to those requests.

### Cronjobs

Services would make high use of cronjobs. Instead of the usual cronjob syntax, we use arc syntax. Its still possible to use legacy syntax, but you must convert it first.

```toml
# cron.toml
# would be saved to somewhere and be read by spx:cron

[name_of_cron_job]
daily = true
times = [
    "11am"
]
scripts = [
    "/sys/scripts/name_of_script.rsh"
]
```

### Userspace vs Kernelspace services

Code running in kernel space should be mostly service handler calls that dictate mechanism, not policy. Otherwise the principle of least concern and modularity / data flow may be negatively impacted.

Some code like the fs service could prob be a kernelspace only service. Its code could be considered privileged code, and have the ability to directly call kernel handlers. But IDK, that might be harder to manage overall. Putting it in userspace as any other process but with flags like `spx`, and predetermined privileges for what its doing sounds like a good idea. So if you code it wrong, it wont break the system, it will just be terminated by the kernel itself or the shell supervisor.

The code in the kernel should be very secure and not doing too much. Attack surface and stuff too.

## Programs

All userspace programs should be an ELF file with `+x` program permissions. There is hardly any way for programs to execute themselves. Unless you use a browser or non verified app. You are exposing your system to anything if you use an unofficial browser or app. Esp if its not open source so it cant be checked anyway. The verification comes in two levels: full and checked. A checked app must be either provided by the devs if its closed source or the code provided as an open source repo. Tools will be used and manual checkers to gauge the code quality and if anything looks weird. The commit must be satisfactory to the checker group. Each time you publish a new version, you must provide a commit history to be checked again. Otherwise the new update wont be checked and the default version will stay the same.

If you distrust a program, you can revoke execution and other permissions from it. All executable files have a list of permissions in `/sys/permissions` or on the file itself. It cannot change its own permissions metadata.

If a program does not have the permissions it wants, it would either have to detect that and try something else, keep going without it, or prob crash/exit. A properly coded program should check the `Result` to see if it got the result it wanted. If not, its prob cause of permissions. Then it would `exit(FAIL)` and say it couldnt do what it wanted to do.

### Inheritance of Permissions

A child program may be granted the permissions that its parent has. By asking its parent. If the parent agrees, the the child will also gain the permissions. This is of interest with `spx:system` which has all the permissions available to it. `spx:arc` should be granted most if not all the permissions. And apps that run as a child of `arc` like a terminal emulator running a program, a browser, etc. must ask `arc` for the permissions.
