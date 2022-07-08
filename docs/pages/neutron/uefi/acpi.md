---
layout: default
title: ACPI
parent: UEFI
grand_parent: Neutron
---

## Advanced Configuration and Power Interface

This thing made by intel, microsoft, toshiba, etc. It defines a hardware abstraction between the firmware (UEFI) and the hardware. Now it works for BIOS too but is less BIOS centric.

ACPI is now actually owned by the UEFI forum. So uhh..

The good thing about ACPI is that you have:

1. an 'ACPI BIOS'
2. ACPI tables
3. ACPI registers (load/store)

The best thing about ACPI is that you have a mostly platform independent way of describing hardware. So that you can port an OS that uses ACPI across multiple hardware systems.

The 'tables' are mostly stuff like RSDP, RSDT, MADT, etc. These are described in 'AML', ACPI machine language. So you can write a parser for them. I think they're mostly c-like anyway and usually copied into RAM. So if you know the RSDP you know everything else. Actually Idk if they're copied into RAM. Maybe they're meant to be stored on disk or system ROM.

Most systems like motherboards are pretty 'non-expandable' in terms of adding completely new controllers and ports. You can plug in stuff onto existing ports and kinda simulate expansion ports with a dock or something. But then that relies on the dock and kernel drivers/user drivers.

## Kernel ACPI Component Arch

ACPICA is a cool platform independent arch. I think you can include it into the kernel or something.
