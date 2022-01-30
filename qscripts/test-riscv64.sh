#!/bin/bash

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"

# compile rust
mkdir build
cargo btrv
# assemble assembly
$AS -c support/arch/riscv64/asm/entry.S -o build/entry.o
# link objects
$LD -T support/arch/riscv64/link/linker.ld -nostdlib build/*.o build/*.a -o build/kernel.elf

# clean up
if [[ $# -eq 1 ]]; then
rm -rf build
rm -rf build/entry.o
fi
