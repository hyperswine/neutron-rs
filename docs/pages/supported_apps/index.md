---
layout: default
title: Supported Software
has_children: true
---

## List

### Vital

rustpython -> can be compiled to wasm32-wasi

cargo -> supports the riscv64gc-neutron-elf and wasm32-wasi targets

- includes rustc, most useful modules of `std`

servo -> compiles natively to riscv64gc-neutron-elf

strictyaml -> required for many system config files. Runs on rustpython

toml -> rust toml parsing library. Used for some system config files

nvm -> managing node installations

- npm & node -> packaging node projects and a runtime for javascript
- electron -> required for browser engine based apps like vscode and discord
- other JS/TS tools like babel, webpack, etc.

### Useful

spacedrive -> filesystem that works on many backends like neutronvfs

krustlet -> wrapper around kubernetes on open containers to schedule wasm clusters/nodes/jobs

thrussh -> ssh implementation 100% in rust

## Custom Targets

The riscv64gc-neutron-elf target is not official. It is simply a `.json` file:

```json
{
  "arch": "riscv64gc",
  "data-layout": "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
  "executables": true,
  "linker-flavor": "ld.lld",
  "linker": "rust-lld",
  "linker-is-gnu": true,
  "llvm-target": "riscv64gc-unknown-none-elf",
  "max-atomic-width": 128,
  "os": "neutron",
  "panic-strategy": "abort",
  "relocation-model": "pic",
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-pointer-width": "64",
  "disable-redzone": true
}
```

## Running Software on Neutron

Once neutron is booted and `init.elf` has been completed. A shell or some form of UI should be available to the user.

```bash
user@localhost ~ $
```

Here a user can run any supported executable through `exec <path_to_file>` or simply just `./<path_to_file>`. If the path of the file is already in path, then it can be run with just `<filename>`.

All this does is call `spawn_process(path, flags)` from the current process (shell, which would be a child of the tty process, which would then be a child of `__sparx_init`). By doing so we fork the shell's open file descriptors and create a new process container. If the program is actually executable that is.

The program should be an ELF file that is loaded by kernel space routines that also handle memory management and fetching from disk/memory if the file is memory mapped.

There is no scanning of executables before execution. Just a scale of `1-10` of how confident the user/OS is in the identity of the author and the file's contents. Files that have a verified status can also be executed directly in Arc.

By default, neutron does not provide any extra indirection or checking for file integrity. If you try to run an executable file in the default root shell. Then it runs pretty much directly according to how much the file wishes to allocate. The scheduler and memory manager should prevent a process from hogging too much execution time. But not allocation. So on Arc, verified executables must show that they aren't doing anything too problematic or weird like insane memory usage.

### ELF Loader

There is an [elf loader](https://github.com/gz/rust-elfloader) made in rust that seems to work well. The format is pretty standard and non volatile so its prob not too hard to do well.

All executing a program involves is creating a process container which has its own file descriptors and page tables. Which the kernel add to its bookkeeping.

Then the specified ELF file can be DMA'd from disk to memory as a memory mapped file so the kernel routines can easily analyse its contents. Then load the segements it requests, any `db, dd, dw` requests and view its program header for the entry point to pass off to.

[Goblin](https://github.com/m4b/goblin/blob/master/src/elf/mod.rs) seems like a more configurable idea and works on `no-std`. So it can be compiled along with Neutron. Kernel routines can use it to parse an ELF file and load things on the fly with other kernel routines.

## SSH

OpenSSH or something like thrussh can be used to do ssh operations.

On rei-shell, it is possible to `ssh` directly into any other device, where one will see:

```bash
user@<remote_url> (ssh) ~ $
```

where rei-shell displays extra things by default for clarity. This can be turned off easily with `/sys/config/ssh.yml`.

## Neutron Sparx

Sparx are cool little processes that run in the background. And can be seen with `htop` with the label `__sparx_` and status `bg`.

## Rust

Rust is a first class citizen of Neutron. Although low level syscall interfaces are written in Phantasm, Rust is used to interface with most things in userspace.

### Rustup

Rustup is the default and best way to install rust on Neutron.

To build rustup for neutron, one should cross compile for the `riscv64gc-neutron-elf` target. The resulting executable can then be executed directly on a neutron build.

### Cargo

Cargo is what makes rust so good, at least in my opinion. Rust is already quite good in terms of its semantics and strictness. But Cargo's build tool has many cool features like unit/integration testing, feature flags, automatic dependency fetching, directory structure and project structure.

Rustup installs cargo and rustc, so they must be able to build on neutron. Cargo itself isnt too problematic to do, but rustc is a different story.

### Rustc

The core of rust. It compiles itself so a working rust install should be able to cross compile for neutron. Although `std` may not be implemented, which makes it less useful as we cannot access OS services in a neat way.

Though for now one can still use `neutronapi` to do so. Cross compiling will also be a big thing for the most part since rustup/cargo will also rely on `std`.
