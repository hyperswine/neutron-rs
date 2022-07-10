---
layout: default
title: Boot
parent: Neutron
has_children: true
---

## Boot Configuration

Neutron can be configured to boot as an:

- arcboot kernel
- multiboot kernel
- simple kernel (efi stub)

Each config assumes an EFI compliant firmware for arm/riscv.

The multiboot config is able to boot on grub2/uboot/oreboot.

The simple kernel config has a built in boot stub for an efi-like boot. It masquerades as an PE (has an extra PE header) and is placed in the default system efi boot dir.

## U Boot

U Boot can also be configured as low level firmware for something like a pi 4. It can completely replace the on board NVRAM that stores the default BIOS image. Prob a good idea to use as an open source alternative. And also for tinkering. There is even UEFI support.

Which begs the question, why not just compile U Boot to a firmware image for a specific board, with UEFI support. Idk if thats possible, prob not. One will have to build EDK II for PI 4. And include the properietary firmware images for the network interfaces. That firmware is actually used by the kernel to drive the network cards. So its more like 'driver software' rather than actual Option ROM.

## UEFI

Uhh. Its not a terrible idea I guess. But it screwed with my page tables it seems. I thought it would just ID map 100% of the stuff. But maybe it doesnt. So that was really bad. It mapped 0x4000_0000 to 0xBFFF_0000 it seems. Or maybe its just rust?

Im not sure whats going on. I started the heap at 0x4000_0000. But it actually printed 0xBFFF_0000 as the "address" of the first string allocated. When I try to read to 40M hex, it actually does read the right data. uh what??

Maybe that address was somehow processed. Like 40M hex. Would make sense since TTBR0 was populated. What does reading the address of a variable actually do? Does it read the virtual or phys addr? Why did it return the phys addr if it is the phys addr?? In userspace with an OS, it should be the virt addr. Maybe it wasnt setup properly?

## Device Tree

Device trees are ubiquitous in RISC. Arm and Riscv systems would usually have precompiled versions available. I think most firmware should be flashed with the fdt itself somewhere in ROM or disk. When the system starts, it would copy it to memory somewhere. And depending on the bootloader and its own boot protocol, it will copy the fdt to a specified point. Its kinda annoying because I dont think theres a standard place where its copied. In uboot it somehow finds it and attaches to your payload. So if using uboot as a stage 1 bootloader or perhaps the firmware itself (given you know what device you are flashing to) its prob ok.

The device tree contains key info like where DRAM starts, what device controllers are attached (and what devices are attached? Maybe uboot could setup a listener on those ports and update it). One of the things with it is the fact that you have to poll the controllers mostly. To wake them and to see what devices are connected. If possible, subscribe to them with interrupt handlers. So when a device is plugged in/out or changes state or receives/sends info, you have pretty much instant and custom access to it.
