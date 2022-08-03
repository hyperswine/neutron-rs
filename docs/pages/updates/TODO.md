---
layout: default
title: TODO
parent: Updates
---

## List

- uhh, build ontop of arcservices where possible. For things like memory regions. But dont rely on it to be somewhat freestanding? Nah I like a bootloader, a boot stub would make things messy. So what should be abstracted? Memory, access to devices through memory, a device tree in the form of arc devices cross-arch. You just have to read/write to it. Arcboot does all the heavy lifting (?) no it doesnt. It does all the wrapping to make the interface uniform across arches. Like logging to serial out, like the framebuffer device. Like the gpu device. You can interact with it by reading/writing to it through arcboot methods (zero overhead, inlined) without checking for arch
- get it **working** on aarch64 with something at least pi 4 to current arm (v9??)
- make rust wrappers around the VC6 and valhall kernel drivers and maybe wifi/ethernet. Also specifically the multimedia card has stuff like crypto and compression which I'd like to use

What is working?

uh.. Idk really. Like it can boot the full way through, firmware -> bootloader -> neutron. And go into a software console (tons of 2D software libs). Before that, some background services should be started like `spx:system` which becomes the parent of everything else. The kernel is not the parent, but does have pid 0 (mostly for TLB ASID). Problem is when you start deleting processes, you gonna have to flush out those specific entries to reuse them and push that pid onto the stack

Technically you dont even have to be in userspace. But its a good idea I think. At least provide a println! implementation of use neutronapi to not just make syscalls but have access to higher level framebuffer or direct terminal protocols. You make a process. And if that process is part of a parent, it can use that parent's devices?

No, with ArcDE you are using WGPU to render.

Err no. stdin/stdout/stderr are temporary virtual files on a per-process level. They are mounted on /dev for reference, so you can "open" the "device" that represents it. What actually happens is the kernel creates an anonymous pipe for all of these file descriptors. Which pipe data written to/from them to the other end. So if you had a software shell process with a builtin console and you write something to it. You are writing to its stdin from `spx:kb`? (Or maybe it kind of does it more naturally, through waker input)

So ok, your kb generates interrupts that become scancodes, which may be "pushed" out to listeners like /dev/tty, /dev/ttyS, or the dev X11 uses. So the scancode gets copied into the processes that are subscribed to those devices. On ARCDE, they must be foregrounded to listen, or have bg perm!!??

For proper interrupt handling, need `irqbalance` and NUMA map. Then you assign hardware interrupts using distance anmd bandwidth hueristics. The closer and wider the bus, the better. I think it usually makes sense to have a wider bus, but one of the problems is the bus parallel signal interference

### Service Calls

They dont need to be handled right away. They simply change the execution context and save the user thread state to the kernel stack. Technically, if rust/llvm did it properly, you dont need to save all of them if you treat a syscall like a function call. Not too mention L1 caching on write-back pages. And if you mapped the kernel's vaddr space to the process already, then you need to use TTBR1 instead in EL1, which should be automatically. So your saving at most 8-16 registers in a mostly sequential way (if its possible to do this with double saves that would be nice), which would be write-back 90% of the time, and you dont need to flush TLB, just change your ASID to 0, which should be automatic since your in EL1, where all pages in TTBR1 have ASID 0.

Now you may not want to handle it right away. This may be because there is a pending hardware interrupt or other kernel threads waiting on each cpu queue with as high or higher execution priority. Thats fine, the kernel mode should treat the service call as just another kernel thread. And on each tick, either take it off the queue for execution or requeue it. A service call may be interrupted by the scheduler. Really anything can be interrupted by the scheduler, except double and triple faults.

Interrupt priority:

1. Triple
2. Double
3. Scheduler
4. Hardware non maskable
5. Hardware maskable
6. Software instruction/memory exception
7. Software service call

All kernel threads queued should get a chance to execute. Neutron uses randomisation + time waited + priority level hueristics to determine which kthread should take over the core next. The scheduler runs on each core and can be modified to suit that core. E.g. core0 is usually pinned to the shell, and might not need as complex scheduling as the other cores. Cores 1-3 are also high efficiency cores which are usually assigned to low power threads, background services and hardware/io interrupts. Cores 4-7 are high performance cores meant to stick to their task as much as possible and change on the fly to get the application going at full speed.
