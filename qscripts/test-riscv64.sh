#!/bin/bash

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"

# clear
clear

# compile rust
rm -rf build
mkdir build
cargo btrv
# assemble assembly
$AS -c support/arch/riscv64/asm/entry.S -o build/entry.o
# link objects
$LD -T support/arch/riscv64/link/linker.ld -nostdlib build/*.o build/*.a -o build/kernel.elf

QEMU="qemu-system-riscv64"

# multiplex the virtual serial port (UART) and QEMU monitor into stdio (UART0)
$QEMU -machine virt -bios build/kernel.elf # -serial mon:stdio
