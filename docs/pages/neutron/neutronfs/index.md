---
layout: default
title: NeFS
parent: Neutron
has_children: true
---

## NeFS - Neutron Filesystem

Similar to apfs and btrfs. But minimalised and optimised for neutron syscalls, services and quantii apps.

### Problems with Current Hierarchical FS and Hardware

- Wear levelling of flash storage
- Advanced features not of much use to the average user. Could perhaps be done in userspace
- Features like multiusers and permissions not really useful on most personal devices
- Encryption not really that good esp if an actor gets ahold of the hardware anyway. They can help in some cases. Like vaults and timed access though. But usually not really that useful, rather focus more on hardware security and alarms / failsafes / backups
- Faster and leaner backups needed. Can take its time in the background every week. Scheduled by `__sparx_cron`. And ultra high compression ratios (or choice between standard and ultra high on smaller storage sizes/full storage) using XZ and multiple runs. For most use cases, gzip is fine
- No need to optimise for disk seek latency. Just read as hard as possible and write as hard as possible

### Flash Storage

We do have to explicitly erase memory blocks before they can be written to. If we are writing a lot of data, that could be a problem and we would have to wait a while for it to complete. So it is a good idea to erase unused blocks while the device is idle, e.g. `__sparx_cron` and `__sparx_disk`. `__sparx_neutron_filesystem` mostly handles other stuff like ref counts.

### Features

- Optimised for ssds (pcie 4.0 and beyond) and native 64-bit
- Scalable volumes from embedded to HEDT
- Fast dir sizing
- Atomic save primitives
- Encryption and checksums

### Single User Design

BTRFS stores extra metadata for each file, on the file itself. It isnt too big of an issue but what if you are mostly the one using the system?
NeFS is designed to be main user first. Extra users are supported with userspace utilites and extra bookkeeping, which can be made on the fly. Rather than built into the filesystem.
