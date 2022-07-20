---
layout: default
title: Memory
parent: Neutron
---

## Memory Management

By default, we use 4K paging and a 48bit -> 48bit addr space/translation scheme. The kernel is ASID = 0 and mapped to the higher half, with all addresses beginning in 16 `1`'s being mapped there. If using ARM, that means TTBR1 would be used for that addr, which the hardware MMU handles. If using riscv, you would use the same page table.

Technically, both arm and riscv has the same number of page tables for n processes (including the kernel's). But in riscv, you have to go the extra bit to remap that table to each userspace table. Actually, idk, I think its basically the same. Maybe have `PageTable` where you have `kern_pt` and `pt_process`. Where `pt_process` has an associated `process_addr_space`. Then you can `process_addr_space.map(kern_pt).map(pt_process)`. If the page tables were setup properly and dont overlap, then it should be fine. For riscv, you need to somehow point the entries of a userspace pt to the kernel's, since the addresses dont automatically use the kernel's pt.

## Virtual Memory Object

The `VMemObject` is a page aligned, contiguous object that represents an underlying region of physical ram.

```rust
type Err = &'static str;

fn request(args: Args) -> Result<R, Err>
```
