#!/bin/bash

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"

# cleanup prev builds, note, all of them (can do build/riscv-virt, build/spectro, build/pi4 later)
rm -rf build
mkdir build

# compile rust code to library
cargo brv && echo "DONE COMPILING RUST"
# assemble "bootloader" assembly
$AS -c sparx/arch/riscv64/asm/entry.S -o build/entry.o && echo "DONE ASSEMBLING ASM"
# link objects
$LD -T sparx/arch/riscv64/link/linker.ld -nostdlib build/*.o build/*.a -o build/kernel.elf && echo "DONE LINKING"

QEMU="qemu-system-riscv64"
echo "STARTING UP QEMU..."

if [ $# -eq 0 ]; then
    echo "No arguments supplied, running with default params"
    echo "RUN: qemu-system-riscv64 -machine virt -bios build/kernel.elf"
    $QEMU -machine virt -bios build/kernel.elf
fi

if [[ -n "$1" ]] && [[ "${1#*.}" == "serial" ]]; then
    echo "serial supplied, running with -serial mon:stdio"
    $QEMU -machine virt -bios build/kernel.elf -serial mon:stdio
fi

if [[ -n "$1" ]] && [[ "${1#*.}" == "virtio" ]]; then
    echo "virtio supplied, running with -m 512M -serial stdio -parallel none -display none -device virtio-gpu"
    $QEMU -machine virt -bios build/kernel.elf -m 512M -serial mon:stdio -parallel none -vga virtio
fi

# For info, qemu-system-riscv64 -M virt -s -S -monitor stdio
