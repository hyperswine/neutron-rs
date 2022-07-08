---
layout: default
title: Spectre Hardware
has_children: true
---

## Spectre Silicon

Spectre is a hardware suite of open source hardware designs (schematics, hdl, specs) and firmware that runs on the controllers for those devices.

## Spectre Sokit

The Sokit or "Soc Kit" is the key to any spectre system. The SoC includes an MPU, RTU, unified memory and a whole range of ports for embedded, mobile, desktop and high end compute applications.

### MPU

The MPU has two levels of caching, L1-per core cache and L2-shared cache. Each level of cache differentiates between Instructions and Data.

'Generic Cores' implement RV64GC and Phantasm Generic ISA.
Each core has 64K L1 I and D caches.

- 1x Master Core
- 3x Sparx Cores
- 8x Efficiency Cores

'Compute Cores' implement Phantasm Compute ISA

- 4000x FP32/I32 Compute Cores
- 200x FP32/16 Tensor M2A Cores
- 200x Function Cores

### RTU

The RTU works on FP12 vectors (kind of like a framebuffer of pixels). It applies SIMD in highly parallelised workloads. It implements Phantasm RT ISA and works with SGC. Userspace drivers can translate vulkan into Phantasm RT + Compute code.

- 4000x FP12 RT Cores

### Unified Memory

Spectre Sokit has 8-32GB Unified Memory shared between the MPU and RTU.

### Wireless Networking

Uses the Spectre Wireless system.

- support for 4G and 5G
- Wifi6E capable
- supports BT 5.2

### Image Processor

Support for 3 separate cameras at the back.

### Ports

A selection of ports for media and useful modular things that isnt already included in the SoC.

There is no GPIO, I2C, etc. on the mobile Sokit. Only available on embedded and SBCs.

- USB
- SODIMM (laptop/desktop only)
- PCI (laptop/desktop only)
