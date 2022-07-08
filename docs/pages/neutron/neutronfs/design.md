---
layout: default
title: NeFS Design
parent: NeFS
grand_parent: Neutron
---

## Overview

NeFS is designed primarily to be a flash filesystem based on log structured fs foundations.

Page alignment of 4KiB by default. Partitions are always contiguous.

- Single user. The root user owns every file and can read, write, execute any file

Guest users are simulated through userspace management. The root user keeps a file `users.yml` which contains a list of users and groups each user belongs to.
`groups.yml` has a list of groups which a user can be part of. Each group has a list of permissions. Like being able to run specific programs. Being able to access specific files / dirs.
The dirs are listed like:

```yml
groups:
    dirs:
        "/guest/<user>" :
            - "read"
            - "write"
```

`init` should load these configs into memory and track which users/groups have access to what files. It should also load a root filesystem view and update it on the fly before pushing to disk.

### API

Sample API for driving an NeFS partition:

```rust
// VFS
struct RootFS;
struct Dir;
struct File;
struct Dev;
struct SymLink;

// NeFS
struct SuperBlock;
struct Key;
struct Node;
struct Inode;
struct Block;
struct Payload;
```

### CoW

Anytime a file's metadata is updated, it will not modify the underlying blocks directly. Rather it will create a copy of it, modify that. Then only when the operation is completed, it will point the original metadata at the new copy. The old one may be saved as a snapshot.

### Checksums

Uses the CRC32C algorithm on nodes. Must be padded to multiples of 1024 bits.

File checks through the `neutronfs_checksum_verify` function. Each time a modification/write occurs, a new checksum is generated and stored along with the file.

Only when a `Pass` status is returned, the file contents should be copied into the kernel buffer as a MM file.
