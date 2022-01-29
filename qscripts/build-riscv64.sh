#!/bin/bash

# clean up previous build - not needed
# rm -rf build && mkdir build

# compile rust
rm -rf build/rust && mkdir build/rust
cargo brv
# assemble assembly
riscv64-unknown-elf-as -c support/arch/riscv64/asm/entry.S -o build/entry.o
# link objects
riscv64-unknown-elf-ld -T support/arch/riscv64/asm/linker.ld -nostdlib build/*.o build/rust/*.a -o build/kernel.elf
