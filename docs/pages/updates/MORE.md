---
layout: default
title: More Updates
parent: Updates
---
## Neutron v0 Update Notes

TO CHANGE:

- sparx/ no longer exists. Logic mostly moved into arcboot
- cargo brv is the main build step. This creates an elf object (non executable). It may be possible to just make it an executable ELF file by compiling it as an executable with a no_mangle `_start`. This can then be booted directly from arcboot `j _start`. Even better `j main`, but then we might be missing out on rust runtime/preexecution stages
- spectro neutron uses SBI and ACPI instead of pure MMIO. The MMIO stuff is handled by arcboot acpi tables
- same for arm

neutron v0, completely hands off most things to arcboot. For riscv, relies on SBI only. For armv8.1, relies on ARM stuff.
To install quantii/neutron, one should use a multiboot bootloader that implements SBI/arm.

1. create a virtual disk `disk_0` with at least 10GB of storage
2. run `arcboot install --spectro disk_0` to flash the `kern` img onto the disk. The `kern` img is basically just a directory containing:
kern-spectro/
    libneutron_kern*.a
    arcboot.o
    recovery.o
3. `disk_0` gets partitioned into 3 different filesystems. FAT32, FAT32, BTRFS. The BTRFS partition is used to store quantii's root partition, which stores its initialization scripts and apps that it will load when booted in.
4. The first FAT32 will be the boot partition. It is fast and not going to be changed often. The boot partition contains `arcboot.img`. On UEFI, we may also have extra files in an .iso like image with `boot.efi` or something. It would prob be in `/EFI/boot/bootrv64gc.efi` or `/EFI/boot/bootarm64.efi`

I GOT RID OF:

```rust
// load regions, request_pages is basically kmalloc()

        // let text_pages = request_pages(validated_elf_bin.text_region_size())
        // * Bss = uninitialised data, which should be in a bss region
        // Bss is only good for reducing file size
        // let bss_pages = request_pages(validated_elf_bin.bss_region_size())
        // Data means global vars and static vars (local + global) which should be compiled & mangled to vars in .data
        // let data_pages = request_pages(validated_elf_bin.data_region_size())
        // let stack_pages = request_pages(5 * self.priority)
        // let heap_pages = request_pages(5 * self.priority)

        // give control to the elf code, start at the top of the text segment
        // this_thread.execute(USER_MODE, text_pages)
```
