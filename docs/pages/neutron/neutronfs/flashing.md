---
layout: default
title: Flashing NeFS
parent: NeFS
grand_parent: Neutron
---

## Flashing a live Neutron/Quantii image

When we format a drive for Neutron, we should use several partitions. Technically we can partition at any point in the drive but assume we have a clean 200GB PCIe SSD.

We would download the neutron kernel img and the arcboot img (both ELF, arcboot contains logic from linker scripts/asm to start at certain positions/UEFI). We can package all the logic needed to format and make 2/3 partitions on the USB into the ISO. The flashing utility will then copy the files onto the USB.

If compression is turned on, we can instead copy a compressed kernel img onto the USB and just copy the full arcboot files instead. It should be like any other process, since arcboot contains an `.EFI` file in its own filesystem partition, the UEFI BIOS should be able to recognise that partition (from the GPT entries) as bootable.

- NOTE, the BIOS UEFI/GPT should see the disk in terms of LBA. So 512B chunks. Or maybe 4KiB? Anyway its the reason why the OS also sees things in LBA
- A GPT entry that points to the start of a bootable partition simply has the boot flag turned on. And a UUID to identify the type of filesystem. Maybe NeFS wouldnt be supported so we can just store `/boot` on a FAT32 partition

### Installer

Neutron comes with NeFS drivers loaded by default. Any new Neutron/Quantii installs should partition a drive with a single root NeFS partition that fills up the disk. The installer can also look at other available drives and format them with extra NeFS partitions.

There are 2 phases of installation. The first is the web installer configuration. Where you select kernel modules to link against and build the kernel + arcboot img on demand. It can take a while depending on the features required. Basically runs `cargo build --target <target> --release --features <features>`.
