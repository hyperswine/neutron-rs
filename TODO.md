# TODO

## Les Go

- remove as much as possible. And put in one file if possible esp for multiarch stuff and circular deps
- fix paging, src/memory/mmu
- fix exceptions and CPU boot, like setting the right CSRs (CSSRW) on M mode
- remove all the cfg(feature = "pi4") etc. Get the board name from the device tree id

## Big Stuff

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

## Wanna Develop for RISCV or X86 instead?

The default vscode highlight is aarch64, but you can set it to riscv or x86 for good language service.

```json
{
    "rust-analyzer.cargo.target": "riscv64gc-unknown-none-elf"
}
```

For limine:

```bash
xorriso -as mkisofs -b limine-cd.bin \
    -no-emul-boot -boot-load-size 4 -boot-info-table \
    --efi-boot limine-cd-efi.bin \
    -efi-boot-part --efi-boot-image --protective-msdos-label \
    iso_root -o neutron_kernel.iso

limine/limine-deploy neutron_kernel.iso

qemu-system-aarch64 -M virt -cpu cortex-a72 -cdrom build/neutron_kernel.iso --no-reboot -d int -D qemulog.log -serial mon:stdio
```

To print a target triple:

```bash
rustc +nightly -Z unstable-options --print target-spec-json --target aarch64-unknown-none
```
