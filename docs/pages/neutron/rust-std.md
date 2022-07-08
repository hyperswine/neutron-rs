---
layout: default
title: Rust Standard Library
parent: Neutron
---

## Neutron rust-std

When building (on host or cross compiling) a neutron exe or lib, it is recommended to use `std` for most generic programs.

Only higher performance programs and graphics apps/libraries should require `neutron_api`.

## Implementing std

A lot of the std library seems to be pretty platform agnostic. Some of the unsafe parts may need to be enhanced or modified in order to use `neutronapi` for interacting with Neutron. That is, no libc. I think a lot of those OS's have some form of libc which std/llvm/rustc links to.

Instead of cloning std, one can simply implement it directly with neutronapi and publish it on `neutron-bundle.io` which is basically a registry like crates.io. To build `std` for compiling rust code for neutron, one should specify in Cargo.toml:

```toml
[deps]
neutron-std = { git = "..." }
```

then build for `--target <arch>-neutron-elf`. Instead of running `cargo build`, run `arcutils build --target <arch>-neutron-elf` to build your rust project for neutron. Arcutils has `neutron-std` linked to it already so no need to do the above if using it.

`std` also allows stack overflow protection. So that must be implemented for it to work well.

### What to implement

A lot of `std` is simply wrappers around `core` and `alloc`. A lot of the stuff in `alloc` is simply reexported. But some modules like `process`, `thread`, etc. relies heavily on the OS.

List of modules:

- process
- thread
- os
- fs
- path
- env
- net

There are also a bunch of macros that should be implemented:

- println!, print!
- eprintln!, eprint!
- format!
