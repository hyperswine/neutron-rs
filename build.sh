#!/bin/bash

# clean up previous build
rm -rf build && mkdir build

# compile rust
cargo rustc -- --emit=obj --target riscv64gc-unknown-none-elf
# assemble assembly
riscv64-unknown-elf-as -c support/arch/riscv64/entry.S -o build/entry.o
# link objects
riscv64-unknown-elf-ld -T support/arch/riscv64/linker.ld -nostdlib target/debug/deps/neutronkern-34298c874dc1911e.o build/entry.o -o build/kernel.elf
