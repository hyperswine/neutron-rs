# TODO

1. nefs driver implementation for searching subvolumes. Managing CoW on trees like the subvol tree and extent tree. Also checksum tree
2. link the compiled drivers in build/ and build.rs + wrap around midgard drivers in the driver extension layer
3. kernel mode graphics driver writeup for spectro MPU Compute Cores / Command buffers and RTU Command buffers
4. proprietary wifi and bt drivers (openbsd/linux ax200) inclusion. And open source driver conversion to rust
5. open source drivers for mouse + kb and other common ports and devices. Rust embedded has a bunch of them
6. VFS and Virtual layers for controlling those devices before init. Also allow syscalls to be used on files or URLs

## NOTES

Lifetime categories:

```rust
<'os> => entire duration of the kernel from boot to shutdown
<'service> => duration of a service
```
