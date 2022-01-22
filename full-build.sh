#!/bin/bash

# clean up previous build
rm -rf build && mkdir build

# compile rust
cargo build
# link objects
riscv64-unknown-elf-ld -T support/arch/riscv64/linker.ld -nostdlib target/riscv64gc-unknown-none-elf/debug/libneutronkern.rlib build/entry.o -o build/kernel.elf
