---
layout: default
title: Boot Process (AARCH64)
parent: Boot
---

## Overview

Neutron's default config runs as a bare metal aarch64 ELF program. It assumes that a multiboot bootloader exists and is capable of loading ELF segments into an identity mapped virtual address range 0-2^48 which is then mapped fully into an at least 8GB physical RAM space. In the 'higher half', that is. So that the higher half virt addresses are prob mapped into the starting 0-2GB of RAM. Userspace processes would be loaded in the 2GB- physical RAM space and the lower half of virtual address range.

The kernel is simply just another program. But runs in EL1 mode, having access to all possible instructions in the A64 ISA. It prob makes sense to think of it like a program that arbitrates other programs. It manages the creation, requests and lifecycle of all the other programs running alongside it.

### Higher Half

Arcboot implements the multiboot protocol as well as the arcboot protocol. For each, Arcboot sets up identity mapped paging on aarch64.

After setting up 4K paging, it saves the bookkeeping in a struct:

```rust
struct Paging {
    page_size_bytes: u64
    // offset of the identity map start - physical RAM, e.g. 0x8M
    offset: u64
}
```

The bookkeeping structures can be passed as a pointer to the kernel entry function in a0. Assuming the kernel knows what the structure is via the rust FFI.

The kernel ELF file should specify that it is to be loaded at `0xfff0000000000000` in a per user virtual address range. The kernel sees the lower half of the virtual address range to be mapped to the userspace as a whole. Even though that usually just means for each process (1 process).

### Kernel Image

Neutron is built as an aarch64 executable ELF image. Little endian, no extra flags. Virtual addressing for all segments.

### Paging

Arcboot should ensure that all memory accesses use the paging functions. So all read()/write()s to a memory address should result in implicit CPU system reg TTBR1 lookup. If the requested memory address/page doesnt exist in the page table, a page fault is generated and handled by the code which arcboot had setup.

Note the bootloader is only responsible for loading the required structures in memory for the kernel. So it is then up to the kernel to setup the environment for user processes. This includes TTBR0, which can be setup in a similar manner. The userspace stack always starts at the lower half 0x000F FFFF FFFF FFFF. Which uses TBBR0 / the other system register that points to TTBR0 mapped somewhere in physical RAM.

The good thing about Arcboot is that it is able to map the physical MMIO addresses effectively into kernelspace. The MMIO addresses may be at random places in RAM so we have to wrap around that to not use those frames for general memory. In Neutron, all MMIO should be in the range 0x0-0x1M. Arcboot also provides a nice struct that details this MMIO mapping for each device in the DTB.

### Dynamic Memory

Many useful things require some form of memory that can grow. Lists of fixed size elements, etc.

Neutron uses a slab allocation algorithm for both kernelspace and userspace (TTBR0/1). In kernel space, the heap starts at around 0x1M and grows down towards the stack.

The heap still uses 4K paging on the low level. Though it is able to split up pages into smaller chunks such as 64B, 512B, 1024B, etc. Larger sizes require several pages. Heap allocation like this inherently fragments pages, mostly internally.

Fixed sized heap allocation allows pages to be split and used in fixed size blocks. A fixed size block is multiples of 2, starting at 64B. Each time a process needs to allocate more memory with `mmap`, the kernel finds a suitable sized free block and allocates that for the process (or itself). As we can see, the heap kinda breaks the pages up and circumvents it.

You cant overwrite a heap block that doesnt belong to you. Like any other memory address, the kernel stores metadata on who owns which block (vaddr + size) through a linked list of linked lists. It can be quite slow to check so Neutron also retains a cache of 1000 recently used heap blocks. If a memory read/write is on the heap (below the sp), the kernel checks TTBR0 like usual. Then if the page is a heap page (checked through a bit or another O(1) structure).

IDK. I feel like you should use physical frames. Or IDK.

Or we can just let apps do it themselves. Many programming languages (prob rust too) have their own heap allocation mechanisms using slab allocation on the heap space in process memory. For the kernel, it can just be another section that it offloads to the program as a `Growable` section which can grow up when `mmap` is called. How it grows up or the underlying page meaning, the kernel doesnt care about. The language runtime should provide some form of heap pointer and size bookkeeping. As well as fragmentation/defrag heuristics. It prob should also assume a 4K paged memory but it can always assume that the pages will always be there.

I feel like to do kernel heap allocation well you need either a minimum of 4K per block in multiples of 4K. Or just do it for the kernel only.

## Arcboot

Arcboot is the default bootloader for Neutron.

Features:

- exports a library for Arcboot kernels to use. Including wrappers around certain AARCH64/RISCV64 operations and accesses
- either statically linked to the kernel (must use `_entry` rather than `_start`) or better, installed separately. If installing separately, use the default linker script and config

On RISCV:

- sets up SEE and SBI and allows `ecall` from the kernel through rust-sbi interface
- sets up 4K paging on 64 bit vaddr
- maps 0xfff0... - 100 pages automatically for the kernel stack so no need to allocate pages on the fly

On AARCH64:

- does all it needs to do

Arcboot exports a great little macro `entry!`:

```rust
// arcboot.lib
macro_rules! entry {
    (x:$expr) => {
        #[no_mangle]
        extern "C" fn _start -> ! {
            $x
            loop {}
        }
    }
}

// kernel
use arcboot::entry;

entry! {
    hello_world();
}
```

It also sets up a UART0 serial out at 64xUART0. Which is a single usize memory address that accepts reads/writes. This directly outputs to UART0. On QEMU, this can be captured by stdout via `-serial mon:stdio`.

A console framebuffer is also set up at 64xFRAMEBUFFER. (0x1510 0c10 1505 1444).

### Linker

An Arcboot kernel should set its stack pointer at vaddr 0xfff0...

Its read only sections should be loaded pretty close to the top of the vaddr range, at around -0x40000. This includes the .text and .rodata sections. The first 32bit instruction can and should be executable at -0x40000. Note this is because we want it to be aligned to page size (4K). All segments need to be aligned to 4K.

The RW sections is then placed right after the RO sections. It just needs to be on a 4K boundary. (Although maybe not?)

The implicit sections like the stack and heap can be placed at arbitrary positions given that the kernel image isnt too big. The heap pointer is stored in `hp` and simply points to the next 0x1000 aligned page after the RW section. The kernel implements its own heap management, although I guess the bootloader could set it up too. Maybe Arcboot sets up a slab allocator and passes the in memory bookkeeping to the kernel.

### Bookkeeping

Theres a few structures that can be passed to the kernel. One is an in memory view of MMIO based on the DTB of the system. Another is a pointer to the page tables somewhere in physical RAM. This pointer is actually a virtual address that uses the page tables itself.

Arcboot does not pass on what the kernel doesnt need to know. For most things though, it could be pretty cool for it to know since the kernel is the supervisor.

The main data lies in the vaddr ranges 64xARCBOOT_START + 10 pages. These memory values are fixed and structured like:

```rust
#[repr(C)]
struct ArcbootData {
    heap_start: usize,
    heap_size: usize,
    mmio_start: usize,
    mmio_tree: MMIOTree
    ...
}

#[repr(C)]
struct MMIOTree {

}

// API
const ARCBOOT_DATA_POINTER: usize = 64xARCBOOT_START;
const ARCBOOT_DATA_SIZE: usize = 10 * PAGE_SIZE;

pub fn get_arcboot_data_pointer() -> (usize, usize) {
    (ARCBOOT_DATA_POINTER, ARCBOOT_DATA_SIZE)
}

// kernel
use arcboot::get_arcboot_data_pointer;

entry!{
    let res = get_arcboot_data_pointer();
    // query arcboot data
    unsafe {
        let arcboot_data = res.ARCBOOT_DATA_POINTER as ArcbootData;
    }
}
```

The pages that contain the data are given ownership to the kernel. Allowing RW.

## ARCI

ARCI = A Relatively Cool Interface. A simpler and more "rusty" version of UEFI and BIOS.

## Notes

0xfff0... = 18 Exabytes. So the kernel is loaded at 18 Exabytes of the virtual address range
