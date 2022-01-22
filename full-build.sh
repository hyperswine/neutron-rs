#!/bin/bash

# AARCH64
aarch64-none-elf-gcc -ffreestanding -c src/kernel.c -o build/kernel.o -O2 -Wall -Wextra
aarch64-none-elf-gcc -T linker.ld -o build/myos.elf -ffreestanding -O2 -nostdlib build/boot.o build/kernel.o -lgcc 
aarch64-none-elf-objcopy build/myos.elf -O binary build/kernel8.img

# ARM32
arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c support/arch/aarch64/boot.S -o build/boot.o
arm-none-eabi-gcc -T support/arch/aarch64/linker.ld -o build/myos.elf -ffreestanding -O2 -nostdlib build/boot.o build/kernel.o -lgcc
arm-none-eabi-objcopy build/myos.elf -O binary build/kernel7.img
