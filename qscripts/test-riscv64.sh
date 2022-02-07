#!/bin/bash

AS="riscv64-unknown-elf-as"
LD="riscv64-unknown-elf-ld"

# build the test config
cargo rustc --target=riscv64gc-unknown-none-elf -- --test --crate-type=staticlib -o build/.elf
# cargo test --target=riscv64gc-unknown-none-elf --no-run

# assemble assembly
$AS -c support/arch/riscv64/asm/entry.S -o build/entry.o
# link objects
$LD -T support/arch/riscv64/link/linker.ld -nostdlib build/*.o build/neutronkern -o build/kernel.elf

QEMU="qemu-system-riscv64"

# multiplex the virtual serial port (UART) and QEMU monitor into stdio (UART0)
$QEMU -machine virt -bios build/kernel.elf -serial mon:stdio
