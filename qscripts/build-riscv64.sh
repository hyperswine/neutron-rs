#!/bin/bash

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"

# compile rust
mkdir build/rust
cargo brv
# assemble assembly
$AS -c support/arch/riscv64/asm/entry.S -o build/entry.o
# link objects
$LD -T support/arch/riscv64/asm/linker.ld -nostdlib build/*.o build/rust/*.a -o build/kernel.elf

# clean up
rm -rf build/rust
rm -rf build/entry.o
