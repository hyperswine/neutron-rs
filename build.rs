// build risc v or aarch64
extern crate cc;
use std::env;
use std::process::Command;

fn main() {
    // env::set_var("ASM", "riscv64-unknown-elf-as");

    // assemble the bootloader
    Command::new("bash")
        .arg("riscv64-unknown-elf-as -c support/arch/riscv64/entry.S -o build/entry.o")
        .output()
        .expect("failed to execute process");

    // link bootloader object with the rust rlib
}
