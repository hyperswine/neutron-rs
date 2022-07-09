---
layout: default
title: Interrupts
parent: Design
grand_parent: Neutron
---

## ARM

ARM interrupts are similar to x86. Except 'interrupts' and 'exceptions' are different things.

For syscalls, we have a synchronous exception (0x0) on the VBAR_Eln. This can be generated at a less privileged (EL0) level with the `SVC` instruction.
By doing so, we trap into kernel mode, save our process stack (current thread) or if their is a kernel thread associated with it we should be done already. Just have to save the open file descriptors.

Then we check the 8-bit syscall number in a0. And we jump to the address of VBAR_Eln + a0 to go to the handler function that specific syscall.

To register a syscall handler, one must use the ARM vector table. Basically you store the address of the handler function for that syscall number `i`. In `VBAR_Eln + 64*i`. I think the vector table should be at exactly 0x0 (vaddr) and can be setup by the BIOS firmware or bootloader beforehand so that the kernel can register its syscall routines at those vaddrs.

Once the kernel has control it can make the `SMC` call to go into EL3 to invoke any lower level BIOS/Bootloader Environment functions.
To return to the calling userspace process, call `ERET` which restores the PC from ELR_EL0.

### Neutron API

To register an syscall handler function with number `i`. Use:

```rust
// Type 1 syscalls with 10 args
struct SyscallArgsT1 {
    a0_to_9: [u64; 10],
    stack_pointer: u64
}

// Type 2 syscalls with 5 args
struct SyscallArgsT2 {
    a0_to_4: [u64; 10],
    stack_pointer: u64
}

#[syscall_handler(i)]
fn handler_for_some_syscall(syscall_args_t1: SyscallArgsT1) {}
```

## Per-core Scheduling

When scheduling threads, a kernel-user thread pair is prob used. The kernel scheduler routines should be running beforehand via `init` which loads `__sparx_scheduler` with privilege level 1. This background service is then handed the de facto parent status of all other processes. Every 50 seconds or so on a core, an interrupt occurs which is handled by `__sparx_scheduler`. Like a syscall, the prev states of the processes are saved and kernel routines are run to pop the next process in the queue out. And run from that saved thread of that process.

## System-wide Scheduling

I dunno if its really need to be honest. We have a mostly multithreaded setup. And we let each core determine what it needs.
