#!/bin/bash

# Should extract and refactor functions to arcboot
# as arcboot run --qemu <qemu args>

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"
CARGO_CMD="brv"

if [[ -n "$1" ]] && [[ "${1#*.}" == "arctest" ]]; then
    CARGO_CMD="arctestb"
fi

# cleanup prev builds, note, all of them (can do build/riscv-virt, build/spectro, build/pi4 later)
rm -rf build/*

# compile rust code to library
cargo $CARGO_CMD && echo "DONE COMPILING RUST"
# assemble "bootloader" assembly
$AS -c sparx/arch/riscv64/asm/entry.S -o build/entry.o && echo "DONE ASSEMBLING ASM"
# link objects
$LD -T sparx/arch/riscv64/link/linker.ld -nostdlib build/*.o build/*.a -o build/kernel.elf && echo "DONE LINKING"

QEMU="qemu-system-riscv64"
echo "STARTING UP QEMU..."

if [ $# -le 2 ]; then
    echo "No arguments supplied, running with default params"
    echo "RUN: qemu-system-riscv64 -machine virt -bios build/kernel.elf -m 512M -serial mon:stdio -parallel none -vga virtio"
    $QEMU -machine virt -bios build/kernel.elf -m 512M -serial mon:stdio -parallel none -vga virtio
fi

# For info, qemu-system-riscv64 -M virt -s -S -monitor stdio
