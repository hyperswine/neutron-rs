# Design

Neutron in its default config is a multiboot kernel that can be booted by Arcboot and many other bootloaders.

- It is a "minima" kernel in that the core module only does the bare minimum of what it needs to do, and hands off other stuff to userspace utilities or loadable kernel modules

## Drivers

Neutron itself has quite a few drivers for most peripherals to use in the main shell and NeutronWM. It also implements WASI for wasm containers.

### FS Drivers

Neutron supports NeFS and FAT32. In userspace, it has WASI support for any semantic fs applications.

The main VFS is always `/` and assumes at least one NeFS partition. Extra partitions can be mounted in `/mnt` from mountable drives in `/dev`.

- no caching for higher performance on removable drives. Safety is prioritised over any minor gains in speed

### ACPI

Neutron relies heavily on ACPI for device discovery and loading suitable drivers and exposing those devices to be manipulated in `/dev`.

- uses [acpi-rs](https://github.com/rust-osdev/acpi)

### Hot Plugging

- supports hot plugs. Will load and unload devices on the fly/each cpu cycle if an interrupt occurs for a plugged in/out device
- userspace apps/code using a plugged in device will prob be interrupted and aborted for safety. So one of the things is to always eject properly
