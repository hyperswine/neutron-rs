#!/bin/bash

riscv64-unknown-elf-ld -T support/arch/riscv64/linker.ld -nostdlib target/debug/deps/neutronkern-34298c874dc1911e.o build/entry.o -o build/kernel.elf