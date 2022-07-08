---
layout: default
title: RISC-V
parent: Arch
---

## Running Software

There are 3 main ways of running software on a riscv system.

The first is directly, using an AEE (application execution env) running on bare metal. Which then provides a usermode ABI for an app to run.

The second is the conventional OS running on an SEE. The SEE exposes an SBI, which is basically user + supervisor instructions for the kernel to use. This allows to control everything in the system like normal, but with extra indirections like virtual memory, thread control, process control. The OS then exposes an ABI for applications to run on.

The third is when you have VMs. You then setup an HEE first from the bootloader. This allows you then to set up a Hypervisor on the HEE using the HBI. These H- elements are kind of like S- elements but have functionalities like IO/memory passthrough to make things faster. The hypervisor itself can be seen as an SEE. Which exposes an SBI for an OS, and so on.

### Why use a hypervisor?

An HEE is great for isolating the hypervisor code from the hardware. So you still want some level of abstraction, but closer to bare metal if possible. The SEE will be a layer above where it it is normally.

## Privilege Levels

At any given time, a risc v hart is running at some privilege level. It is a mode encoded in at least one CSR.

When a hart attempts to execute an instruction that it does not have privilege for. It will raise an exception that would usually be handled by the underlying execution environment, e.g. OS, SEE, HEE.

### Trap Routing

Many traps like syscalls are vertical traps as we go down a level then back up. Some traps may be horizontal, where a trap is handled on the same execution level. E.g. a horizontal trap may go down a level, but it then goes back up again to the actual handler that may be isolated from the original application trapping. I guess it could be pretty good for userspace drivers.

RISC-V provides flexible trap routing so you can go down a few levels, then back up a few. Or stay in place. Im pretty sure instructions like `syscall` should always trap down to the SBI if you are in user mode though.

## Debug Mode

We can implement a debug mode to support off chip debugging or testing. This is called 'D mode'. Which is basically another privilege mode with more access than M mode.

So D mode spec tells us what we can do with a hart when debugging. E.g. reserve a few CSR addresses only accesible in D mode. And maybe some portions of the physical addr space so you shouldnt map to them in your paging.

## Control & Status

Control and Status registers is an specified in the extension Zicsr.

There are 4096 CSRs that are possible. In the spec, theres only like 100 defined registers rn.

The 'SYSTEM' major opcode encodes all privileged instructions in the ISA. The instructions can be divided into 2 classes: CSR modification instructions and non-CSR modification instructions.

An implementation may contain additional CSRs not defined in the ISA. These are also accessible by some subset of the privilege levels using the CSR instructions.

CSRs function differently according to which privilege level a hart is running on. Some privileged CSR instructions are also associated closely with a particular level.
Although CSRs and instructions are associated with one privilege level, they are also accesible at all higher privilege levels.

### Conventions of Mapping

So we have 12 bits of 'encoding space'. The highest mapped CSR is number 0x7B3 with debug, read, and write privileges.

Each CSR has an associated privilege, `[M|H|S|U][R][W]`. If readonly, then RO. If read write, RW. We use 4 bits to store this info. If you dont have permission, then it raises an illegal instruction exception.

Also some regs have only some bits writable. So if the reg is writable but you write to a non writable bit, it will just be ignored.

## Notes

Each riscv core should implement those registers. Basically each core should implement all registers and instructions defined. Though its not completely necessary if you want some cores to do some things. And other cores to do other things. Maybe some cores dont need special functions or CSR. E.g. an mpu (main) core vs an ppu (parallel) core.

Standard CSRs do not have any side effects on reads. But may have on writes.

Some instructions implcitly read from a CSR. All supervisor level instructions read from `satp` when loading/storing with paging on.
