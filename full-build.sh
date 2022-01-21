#!/bin/bash

# clean up previous build
rm -rf build && mkdir build

# compile rust
cargo rustc -- --emit=obj
# assemble assembly
riscv64-unknown-elf-as -c support/arch/riscv64/entry.S -o build/entry.o
# link objects
riscv64-unknown-elf-ld -T support/arch/riscv64/linker.ld -nostdlib target/debug/deps/*.o build/entry.o -o build/kernel.elf
