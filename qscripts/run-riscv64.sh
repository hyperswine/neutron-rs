#!/bin/bash

QEMU="qemu-system-riscv64"

# multiplex the virtual serial port (UART) and QEMU monitor into stdio (UART0)
$QEMU -machine virt -bios build/kernel.elf -serial mon:stdio
