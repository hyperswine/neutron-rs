# Neutron
A rust based kernel built on first principles

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
