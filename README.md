# Neutron
A rust based kernel built on first principles

# Testing
A core idea is TDD. The only way to really verify that things work. Each `mod.rs` should contain cfg() for build and test, esp for arch dependent code. For arch independent code, should try not to rely on arch dependent code. I.e. only arch dependent code should rely on arch independent code. Or the interface should be isolated, i.e. in kernel manager.

# Idea (Updated Jan 23rd)
- Make a rust based binary program that runs on bare metal. I.e. no_std
- has all the modules including drivers and etc
- memory management not really cause theres already stuff for it and we want to support both riscv and arm

# Dependencies
Rust (rustup recommended)
 - rust-src
 - target aarch64 and riscv64 (unknown-none)
QEMU

# Building
`cargo build`

# Running
`run.sh`
