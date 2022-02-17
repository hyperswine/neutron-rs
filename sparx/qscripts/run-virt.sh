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
$LD -T sparx/arch/riscv64/link/linker.ld -nostdlib build/*.o build/*.a -o build/kernel.elf  && echo "DONE LINKING"

QEMU="qemu-system-riscv64"
echo "STARTING UP QEMU..."
echo "USING: qemu-system-riscv64 -machine virt -bios build/kernel.elf -serial mon:stdio"

# multiplex the virtual serial port (UART) and QEMU monitor into stdio (UART0)
$QEMU -machine virt -bios build/kernel.elf -serial mon:stdio
