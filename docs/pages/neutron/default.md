---
layout: default
title: Default Config
parent: Neutron
---

## Overview

The default config for Neutron uses:

- Compiles to an ELF64/Aarch64 img
- Fair scheduling of userspace processes
- NeFS root dir with default hierarchy. A single EmberFS partition. A single EFI system/boot partition containing Arcboot and boot config plain text files
- ArcWM and ArcDE based on gnome / KDE with a handful of packages installed
- Rei shell on tty0 with the `terminal` app. Which can be invoked from the DE with `super + T`

The default config for Quantii/Neutron:

- Everything in Arcboot and Neutron, except no Arc or init
- Wasmer runtime running as the first neutron userspace program. And takes over as a pseudo hypervisor (in U mode). `quantii-init` is run as the first wasm program
- Qiish is exposed on tty0 on the `terminal` program
- ArdakuDE is started and takes over most operations. Listens to mouse/KB/other devices. All Ardaku apps, including terminal and etc, are actually children of ArdakuDE rather than `quantii-init`
