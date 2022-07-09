---
layout: default
title: Arm
parent: Arch
has_children: true
---

## Overview

ARMv8 and the latest ARM architectures are supported by Neutron. More specifically, Neutron supports Pi 4 and Rockchip/Quartz Hardware.

### Naming

AARCH64 => means addresses are held in 64 bit registers. Instructions in the base ISA can use 64 bit reg for processing. Supports the A64 ISA

### Arch Profiles

A => Application. Supports a virtual memory system arch. Based on an MMU. Also supports A64, A32, T32 ISAs. So if you are implementing an Armv8-A on a CPU, you can call it an AArchv8-A implementation

R => Real Time. A32 and T32. Mostly for higher protection. There can be a MPU (memory protection unit) to go along with the implementation

M => Microcontroller. T32 only. Designed for low latency interrupt processing. With hardware stacking of reg and support for writing writing interrupt handlers in higher level languages like python

## Generic Timer

We have secure and non secure timers. For EL1 and EL2 only. Also a 'virtual timer'.

You can access them by reading from the [control registers](https://developer.arm.com/documentation/100095/0003/Generic-Timer/Generic-Timer-register-summary/AArch64-Generic-Timer-register-summary?lang=en). They are updated each time the system counter ticks up. I guess there's always going to be some latency reading the val and using it in an EL0 algorithm.

Maybe e.g. `gettimeofday()` could do that by reading from the control register. Or if possible, try to sync with the network time. Maybe copy the current network time into RAM + extra latency from the network. Then increment your counter from there. Or count your date from there.

## ARMC Interrupts

For peripherals or something. You have 16 possible types. 8 of which can be called on by software. You always have timer, mailbox (intercore), "doorbell", VPU (video processing unit), AXI, RAM address error.

## ARM

A good thing about ARM is the uniform/fixed length instruction widths. 32-bits for the most part. Which simplifies instruction decode. So no WLIW or variable instruction widths like x86.

The ARM arch itself features control over both the ALU and shifter in most data processing instructions. To maximise the use of of an ALU and 'shifter'.

We also have auto increment and decrement addressing modes to optimise program loops. For loops, while loops, etc. So instead of going down to the end of a loop block then jumping back up. You can seem to just simulate the entire loop or something.

Load/store multiple data at a time. Allows us to maximise data throughput.

Conditional execution of almost all instructions. So if you have an `add x, s, t` instruction. You can instead make it an `add<cond> x, s, t` instruction that uses the result of the prev instruction. You have a status reg which stores that info from the prev instruction.

### Exception Types

1. reset => invoked on a power on or warm reset
2. undefined instruction
3. software interrupt => includes syscalls (SWC) and traps on E0-2
4. prefetch abort => a instruction fetch memory abort
5. data abort => a data access memory abort
6. IRQ => normal hardware interrupt (handled by GIC)
7. FIQ => fast interrupt

### Exception Process

When an exception occurs the ARM proc halts execution in a defined manner. An exception vector is a fixed address in memory which the processor goes to when an exception comes on.

## Instruction Types

Like riscv, ARM has 6 major instruction types. Each are 32 bits.

1. Branch instructions
2. Data processing instructions
3. Status reg transfer instructions (cmp)
4. Load & Store instructions
5. Coprocessor instructions
6. Exception generating instructions

All instructions have a 4 bit condition field. One bit is to the core to execute the instruction unconditionally.

### Data Processing

The stuff in the ALU and FPU. Bitwise operations and comparisons, SIMD, multiply/divide, simple arithmetic.

### Status Reg Transfer

These ins transfer the contents of the CPSR/SPSR to or from a general purpose register.

We can set the values of condition code flags, interrupt enable bits, set processor (one core) mode and state. Also allows us to alter the endianess of load/store operations.

### Software Interrupt Instructions

Mostly `SWI` for userspace -> kernel. When you want to access some non-trivial system resource like disk, or some device or allocate more memory.

BKPT should be used for an debugging. It causes an abort exception. If a debugger is installed on the abort vector. It should be able to handle that, e.g. pause on that breakpoint.

Even better if debug hardware is on the system. We can treat BKPT directly as a breakpoint and prevent the abort exception from occuring. IDK it may still cause an exception or just handle it like a function `loop{}`, depending on its implementation.

## Data Types

- B => 8 bits
- HW => 16 bits
- W => 32 bits
- DW => 64 bits
- QW => 128 bits

## Very Cool, ARM

ARM has atomic load/store instructions just like RISCV. So we dont have to lock the whole system for mutex/semaphore implementation. Just use those instructions.

Coprocessors can be attached to the main processor pre arm v8. It was a low latency way of adding extra functionality and to offload high performance tasks from the main instruction processing processor. And you could address them pretty nicely with CPR instructions. But AARCH64 does not support them. Cause we have SMP and SIMD so why would you need it?

## ARMv7 Virtual Memory

For 32-bit systems, you have 4GB of addressable memory. I think its actually 8GB. Idk though. Each L1 entry describes 1M of virtual memory space. Then we have L2. Both use short desc table formats. This means page descriptors are 32 bits, max 2 levels of tables, 32 bit addresses only. Support for 16M/1M segments or 64K/4K pages.

For 32-bit, we have 2^12 pages at most. So the first 12 bits tells us the page number and the next 20 bits gives us a 2^20 bits. We can address 1M with it.

### 32-bit Translation

We hold `&L1Table[0]` in CP15.c2. This is the coproc reg. The address must be aligned to 16K (0x4000), so you can start growing the table down at 0x8000000 or something.

When the MMU performs a translation, it automatically walks the table for us and fetches the actual RAM data.

The L1 table just stores entries to L2 tables. It stores 16K worth of data so the L1 table can be stored in a single 16K page. Well I dont think the page tables are paged themselves at least in a non recursive build. So its just kinda invisible and not meant to be addressed.

I dunno really what sections and supersections are for. A section maps 1MB piece of memory to a physical address. Maybe for hardware MMIO or flash storage or something.

If using L1 only, you can address up to 1M for each entry. So 4GB. Idk how exactly their used for but I think L2 makes more sense for an OS.

### Setting up

The TTBR reg must point to the tables. We [set](https://developer.arm.com/documentation/den0013/d/The-Memory-Management-Unit/Virtual-memory/Configuring-and-enabling-the-MMU) it like:

```asm
MRC p15, 0, ..
ORR r1, #0x1
MCR p15, 0 ..
```

### TLB

Cortex-A cores should have an L1 TLB and possibly an L2 TLB. The exact structure varies between implmenetations.

Theres an I-TLB for caching frames with instructions. And a D-TLB for caching frames with data. Theres also I/D caches which should be searched first. If not found, then we retrieve from memory, which involves searching the TLB for that page.

## ARMv8 Virtual Memory

Mostly like x86. I dunno how free frames are used. I think we can just keep a stack of u64 numbers that represent free frames. And alloc them when needed.

I think we have to page the page tables themselves.

Each EL0, 1, 2, 3 has its own set of page tables. So the page tables that the kernel uses can be different to the one a process uses. Although you still want to "map" the kernel's address space into the process' address space. I guess it just means putting your process addresses at the bottom half and not touching the kernel's stuff in the top half. Or maybe it means to use TTRB0/1 simultaneously in EL0.

The way its set up seems to suggest that you should go into EL2 (and simulate a return to a function). Then set up virtualisation tables so that all the different peripherals, RAM, flash, etc. are mapped properly in cont. neat chunks.

The VA space for EL2 and EL3 are shared in the lower half. The OS can then run in EL2 with an upper and lower half. But better to just map the single space to kernel space in EL0/EL1. And make the kernel run in EL1.

We use TCR_EL1 to set the stuff up. I think it may be possible to directly map that into RAM and peripherals instead of having to set up EL2. Or maybe UEFI already did it for us (runtime services). I know boottime services should. But runtime services prob dont since we have to specify a memory map.

### Physical Addresses

ARMv8.0 has 48-bit phys addresses. So you can have a max 2^48 RAM. But usually only 64/128GB by the motherboard or chipset.

ARMv8.2 has 52-bit phys addresses. Regardless of the phys address sizes, you can prob assume you have around 16-128GB of RAM. And therefore 48-bit virtual addressing should do. Even better, maybe just single level paging with 40 bit (1TB RAM) addresses. Just have to make sure the compiler outputs the correct virtual addresses.

### Address Translation

Each entry is 64-bits. The least sig two bits (little endian) determines the type of entry. So there are 4 possible entry types:

1. Table Entry => points to the next level table. For 0-2 only
2. Block entry => points to a generic block of memory. Could be some MMIO or something
3. Page entry => points to the actual frame's base address. For 3 only
4. Ignored. If the MMU hits this, then it should get a page fault. I think. If the page exists on disk then page to RAM. Otherwise zero a frame? No. depends

The encoding for page and table descriptors can actually be made the same. `b11`. To allow recursive page tables. This allows you to point back to yourself so the MMU doesnt have to go to another frame for the next table. You can also calc the virt addr of a particular page table entry. So you can easily update it. Oh so you can page the page tables maybe.

Also all Cortex-A cores support 4K and 64K pages. If using a 64K page, only 3 levels. And you can use 52-bits. You cant use 52-bits with 4K pages.

### TLB Flush

Say you want to flush all ASIDs, when starting up for example:

```asm
STR  X1, [X5]
DSB  ISH            // Barrier instruction to force correct order of execution
TLBI VAAE1IS  , X0  // Invalidate VA specified by X0, in EL0/1, for all ASIDs
DSB  ISH            // Barrier end
ISB
```

Synchronising context is needed to ensure all cores join() up at the ISB instruction.

We shouldnt set the attributes of each entry manually I think. Or maybe we do at first.

Executable pages could prob be stored in the I-TLB. And non-executable (read-only or read-write) could be stored in the D-TLB.

### Disabling the MMU

We can disable the MMU from walking the page tables. We have to disable stage 1 MMU, which should auto disable stage 2. Or disable both of them just in case.

This makes all data accesses Device_nGnRnE. All instruction fetches are cacheable. All addresses are read+write+executable.

We use SSE to order memory accesses. Needed because caches and write buffers are often in the system. We can disable caching I guess. But we can also enforce write-through and sequential memory access to guarentee the data gets accessed and written in the order we need.

### Memory Types

We have: Normal Memory and Device Memory. Device memory are pages or addresses marked as relating to a device in some sort. Maybe MMIO or direct peripheral memory mapping. This doesnt include mapped ROM or Flash.

We also have 'strongly ordered' memory. Which is basically memory that needs to be accessed properly. In ARMv8, this is Device_nGnRnE.

### Fencing

So I think theres some situations where you need to fence. To ensure things go properly. Esp lower level code in general. Driver code should use write through pages / non-cacheable.

### Device Memory

You gotta make sure most of the stuff is non-executable. So the proc doesnt try to fetch instructions from those regions. Device pages are already non speculative accessible dataly.

To ensure maximum correctness, use nGnRnE. For no gathering (bunching up instructions/data then writing it all in one go), no re-ordering (usually its prob not too bad I think, but disk and output display no), no early write (if you just write it to the buffer in the interconnect).

Or nGnRE, which allows early writes. It allows hardware buffers to send an early write ACK to the processor once the data has reached an intermediary step. It also depends on what the device wants. A disk drive prob wants nGnRnE.

To encode this, we use 8-bits in the MAIR. So we actually just need 3 bits to index into MAIR from the entry. As it is split up into 8 regions (of 8B each). The region of interest is region 5. From 0-7.

## Page Attributes

So we have a bunch of them. One of them is the Access Permission attr, which takes up 2 bits. It tells the core whether EL0 or privileged (EL1,2,3) can access a certain page. All pages can be at least RDONLY on privileged mode. 00 and 10 means EL0 cannot access this page, and the core should generate a permissions fault. By default you could prob just handle it by telling the process to go next instruction or just terminate it.

### If using Privileged Access Never (PAN)

Better to not allow the OS to load/store to less privileged EL0 pages. So malicious software cant trick the OS to reading data from other processes. Just set `PSTATE.PAN=1`. This generates a permissions fault. Then you can just terminate the process that tried to make the OS do that and continue your loop.

But you may still want to load/store to unpriv pages. Like copying data to a userspace buffer/set of pages. Then you just use `LDTR` and `SDTR` instructions to bypass PAN. I think most compilers dont realise this so you have to make sure they're used in kernel code.

### Executable Memory

We use PXN and UXN to set the page as executable for the user and/or executable for the kernel. 1 for not executable, else 0.

Note if you set `HCR_EL2.E2H==0` then you can make EL2 and EL3 bypass those bits. So a hypervisor could still fetch instructions from those pages. I think it also allows bypass of PAN, idk, though you can just use LDT/SDT anyway.

### Accessed

The Access Flag bit tells us whether a page has been accessed since creation, orr maybe since some time interval (you have to manually update it). Then on a page bookkeeping tick, you can page out pages that havent been used to swap space.

You can just update the AF in software by looking up the page manually on a page fault or page alloc / mem access. But its better to enable hardware to do it. Available in ARMv8.1.

Theres also a dirty bit. This is useful to tell software whether to flush the page to swap space on page out. Or just ditch it and zero it immediately. This can also be useful for memory mapped files I think. And copy on write semantics. Where you never dupe the frame unless a process wants to write to it, or when it actually does call write() to it. Then you dupe the affected frames into that process' address space and point the pages to them.

For CoW, you prob want the page to be read-only with DBM = 1. This changes the function of AP and S2AP to not care about access faults. And instead record when a process tries to write to it. Then it will change that page to read-write. I dont see how that is CoW though. I think you have to alloc another set of frames and do a memcpy. Then, once the op has been completed, it will want to write back to the original pages.

### Aligned Accesses

Aligned accesses are generally faster and more correct. We always use little endian because its more efficient for the hardware, but you have to specify it in each Exception level. We should always load a 16-bit val with the load16 instruction, from an address that is a multiple of 16 bits.

But we can make unaligned accesses too. At least with normal memory. Not with device memory though. If you dont want unaligned accesses, set SCTLR_EL0/1.A. This makes it generate an alignment fault. Prob a good idea for apps which need to be high performance or extra safe.

## Cache

Caching is very amazing. I like caching a lot. So heres how caching works in ARM. You have data or instructions saved in some bank. Each cache entry contains an ASID. If you want more than 255 processes you can temporarily flush some other used pid's entries and requisition that pid for the new process.

On ARMv6+ we actually tag each cache entry with the physical address. So we need to translate the VA into a PA (that means you might have to fetch from RAM) to use the cache.

## Common Terms

SVE => Scalable Vector Extension. Allows flexible vector lengths instead of fixed 128bit vector lengths in SIMD/NEON. Min val of 128bits to max 2048bits.

PE => Processing Element. Similar to RISCV hart
