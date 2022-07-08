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
