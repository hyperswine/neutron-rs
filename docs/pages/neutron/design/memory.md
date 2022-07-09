---
layout: default
title: Neutron Memory
parent: Design
grand_parent: Neutron
---

## Virtual Memory Layout

Based on: [linux riscv virtual memory](https://www.kernel.org/doc/html/latest/riscv/vm-layout.html). Basically, we have a top half kernel space for managing key infrastructure. And the bottom half is assigned to userspace applications. The virtual memory layout is the same for every process, which all think they are the only process with the only other being the kernel.

On a 64-bit system with 48/52-bit virtual memory, we have:

| Space | Range |
| --- | ----------- |
| Kernel IMG (4GB)| `0x000fffff ffffffff - 0x00000fff ffffffff` |
| Kernel Serviced MMIO (200GB) | `0x00000fff 00000000 - 0x00000fff fee00000` |
| Userspace (200TB) | `0x3f ffffffff - 0x0` |

### Kernel IMG

Coontains the key kernel ELF layout/code for setting up filesystems, drivers, process management, exposing MMIO for key services and syscalls.

Also contains extra kernel modules that can be linked before loading (statically linked). No dynamically linked modules exists. VDSOs do exist in userspace though, for safe operations like `gettimeofday`.

The kernel itself uses virtual memory and is allowed to read/write/execute privileged pages. Assuming that the bootloader has setup virtual memory control registers and passed the memory info (size of physical memory) to the kernel in order for the kernel routines to setup and manage paging.

### Kernel MMIO

Virtual space where driver and low level service ABI can be called by kernel routines. Privileged memory mapped files and device memory usually resides here. GPU Memory too, whether we're using unified or dedicated memory.

## Memory Management

Neutron uses a typical memory management framework with 4K pages, 4 level page tables and Fixed Block/Slab Allocation of heap memory.

### AARCH64

On arm64, we can use TTBR0 and TTBR1 to represent the userspace and kernelspace respectively. TTBR1 is a pointer to the bottom of kernelspace and TTBR0 is a pointer the top of userspace. Each growing in the opposite direction. We can select each one with VA by setting the bits to either all 1 or 0.

Before the kernel is loaded, we should also ensure it is linked at `. =  0xffff000000000000`. This ensures the kernel load address is at the link address and so kernel code executes from there. Then we can start the MMU and stuff when we load paging.

### Paging

For the most part, 4 Level Page Tables and LRU for the replacement policy.

Also must invalidate the TLB on context switch. Maybe restore if possible afterwards. Basically a per-core thing. Though there is also L2/L3 TLBs that are shared across multiple processes.

### Userspace

Stuff like initd, devicemanagerd, moused, cpud, networkd, ipcd, filesystemd etc.

Syscalls should be quite fast if the underlying hardware is fast and kernel subsystems are lean esp scheduling and daemons. I think it also depends a lot on the userspace code implementation. They could be creating another thread for a blocking `read` operation while doing something else in the meantime.

## Notes

On 48/52 bit paging, the max vaddr we have is 2^48 or 2^52. Not 2^64. The TLB should only support 48/52 bits of paging.

So `0x1000000000000` or `0x10000000000000`. Which comes out to the ranges:

- 48 bit: `0xffffffffffff - 0x0`
- 52 bit: `0xfffffffffffff - 0x0`
