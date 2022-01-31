#!/bin/bash

AS="aarch64-none-elf-as"
LD="aarch64-none-elf-ld"

# compile rust
mkdir build/rust
cargo barm
# assemble assembly
$AS -c support/arch/aarch64/asm/boot.S -o build/boot.o
# link objects
$LD -T support/arch/aarch64/asm/linker.ld -nostdlib build/*.o build/rust/*.a -o build/kernel.elf
# ? OBJ COPY to kernel.o actually
