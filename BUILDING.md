# Building

Neutron is built as an ELF64 image using `cargo build`. Note I haven't tried out x86 but Im pretty sure its ELF as well or maybe just a blob.

- the resulting `/build/neutron` can be placed on an NeFS partition on `/sys`
- if using arcboot, it should automatically detect any multiboot kernel imgs using GPT entries
- the magic number of NeFS is `0000-NNNN-FFFF-SSSS`. Arcboot usually prioritises those partitions and boots from the first kernel img found in an NeFS partition

## Commands

`cargo b[arm|rv|x86]` -> builds neutron for a specific arch

`cargo t` -> builds neutron for the host (assumed x86_64) and runs cargo test suite for unit and integration tests



## Dependencies

Rust

- rustup
- targets/rust-src (bare metal)
- llvm-tools-preview

Utilities

- binutils
- arcboot
- gcc (mostly gcc and gdb)

VM

- QEMU
- spectrovm (later on)
