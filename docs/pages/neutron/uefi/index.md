---
layout: default
title: UEFI
parent: Neutron
has_children: true
---

## UEFI on Neutron

By default, neutron is configured with an arcboot loading system. Which uses uefi functions like acpi and efi system partitions for boot. UEFI kernels are able to request services from UEFI bootloaders. On AARCH64, we have SystemReady specs that outline how uefi should be implemented. A DTB can be used in addition to UEFI.

### EDK II

UEFI is an intel/windows thing. A lot of UEFI implementations are properietary and optimised for x86/windows systems.

EDK II is an open source implementation of UEFI by intel. Seems like they wanted to make it more widespread. For more info, check out my [amazing walkthrough of EDK II's source code](edk2.md).
