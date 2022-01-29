#!/bin/bash

# install deps and setup files
mkdir build
cargo component add rust-src
cargo target add riscv64gc-unknown-none-elf

# build for riscv
cargo brv | bash
