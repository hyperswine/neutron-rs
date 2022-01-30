use arcboot::{Arch, Build}

fn main() {
    // assemble
    let build = Build::new(Arch::Riscv64);
    build.assemble("asm/riscv64/boot.S", "build/boot.o")
        .link(&["build/boot.o", "deps/libneutronkern.a"], "link/riscv64/linker.ld", "build/kernel.elf")
        .clean();
}