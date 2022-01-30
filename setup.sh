#!/bin/bash

# install deps and setup files
mkdir build
cargo component add rust-src
cargo target add riscv64gc-unknown-none-elf

##
# SPECTRO
##

# install extra arc tools
cargo install arcboot
# build for riscv64
cargo arcboot build spectro --test 
# run the test cfg. Note, `cargo arcboot test spectro` does the two commands together
cargo arcboot run --test

# when ready, build and run the release config
cargo arcboot run --release

##
# PI4B
##

# build for aarch64
cargo arcboot build aarch64
