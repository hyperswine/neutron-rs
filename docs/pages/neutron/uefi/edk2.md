---
layout: default
title: EDK II
parent: UEFI
grand_parent: Neutron
---

## EDK II Code

EDK II is written in C.

It comes with 28 packages. Each package describes a key aspect of implementing UEFI. Such as a specific arch, arm/x86. The EFI shell, OVMF for qemu, Network support, Security support.

## UEFI/PI Image

UEFI and PI are specifications. They define the standardised format for EFI firmware devices. These are usually flash or other nvram. And are called 'Firmware Volumes'.

EDK II provides a build system that process files to create the file formats described by UEFI + PI.

## EDK Package

A package is a container. It includes a set of modules and their related definitions. Each package is an EDK II distribution unit. It can be used to manage and release the big project to facilitate a user's distribution and reuse.

Entire project sources should be split into different packages to reduce the release granularity. The new project can also be made from released packages from different sources.

## EDK Module

A module is a bunch of source or binary files. With a module definition (INF) file.
An INF file describes a module's basic info and interfaces. E.g. consumed/produced library class/PCD/Protocol/PPI/GUID.

A typical module is a 'firmware component'. You can build it and place it into an FFS file. Then put it into an FV image. A firmware component is either a driver or application. Built into an .efi binary file. At the EFI_PE_SECTION.

Possible FFS files:

- raw data binary. E.g. `Logo\Logo.inf` contains a logo bitmap image
- option rom driver. Placed into a device's option ROM section
- standalone UEFI driver
- standalone UEFI application
- library instance built into a .lib object file. Then statically linked to another module (FFS)

### Module Types

1. SEC => a module like this starts execution at the reset vector of a CPU. They are used to prepare the platform for the PEI phase. Can also produce services that are passed to the PEI phase in 'HOB's compliant with PI spec
2. PEI_CORE => a module used by PEI Core
3. PEIM => modules used by PEIMs
4. DXE_CORE => ...
5. UEFI_DIRVER => provides services in the boot services execution environment. EFI drivers can remain in memory as long as they return EFI_SUCCESS. Otherwise the controller should unload it if it returns an error
6. UEFI_APPLICATION => always unloaded when they exit
