# Neutron
A rust based kernel written with the basics in mind.

### Syscalls v0

- file like handling of most things. Descriptors for files, processes (pid), sockets (all networking), drivers/devices
- also for stuff like virtual fs/images/volumes. And anonymous pipes

## V0

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

## Layout
- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic
- tests/ -> cargo integration tests
- tests/arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now

# Testing
A core idea is TDD. When in doubt, test.

- To run cargo tests, `cargo t` which builds the library for the host target and runs the `#[test]` functions.
- To run vm tests, `arcboot test` builds the a complete image with `feature = arctest` and a custom test harness. It basically runs `rustc test --no-run` for either the spectro/pi target. Then it runs `arcboot run` which boots the vm and loads the image, running the kernel with an arctest config instead of the usual config.

## Cargo Tests
- Mostly for the specific functions (local #[test] in each file) and functions together (things in tests/)
- NOTE: `tests/` is for cargo only and `#[test]` is also for cargo only. Arctest only relies on `feature = arctest` and files in `tests/arctest/`
## Arctest
- Great for validation, blackbox and acceptance testing. Basically any high level stuff that you cant do directly with `cargo t`
- `arcboot test` -> not yet working but will be great for system testing and blackbox testing

# Dependencies
Rust (rustup recommended)
 - rust-src
 - target aarch64 and riscv64 (unknown-none and unknown-none-elf)
 - arcboot
 - spectrovm (later on)
Toolchains (add to path or specify sysroot when using `cargo build`)
 - aarch64-gcc
 - riscv64gc-gcc

## Minimal Config
Since this is a multi target kind of thing in rust, we get a whole bunch of issues if we try to do it the standard way. Recommended to disable any language servers since they can spasm really hard. Maybe theres a way to configure it nicely but Idk I dont really wanna to configure VSCode too much.
- This means things like `.cargo/config.toml` should be very minimal. Mostly for cool things like aliases and stuff. Dont specify any main configs. You can do `[dependencies.X]` for X if you want but I rather leave it mostly vanila and rely on `arcboot` for more complex config and functionality
- Mostly using rust, the language itself and the cargo package management and test suite. I dont really care about the other stuff, at least for now.

## Syscalls II and IPC

neutron service ii is another implementation of syscalls, using kqueue.

Implements the [Ardaku interface](https://github.com/ardaku/ardaku/blob/main/SYSCALLS.md).
Most of the stuff would be in `src/services/ardaku`.
