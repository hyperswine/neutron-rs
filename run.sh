#!/bin/bash

# multiplex the virtual serial port (UART) and QEMU monitor into stdio (UART0)
qemu-system-riscv64 -machine virt -bios build/kernel.elf -serial mon:stdio
