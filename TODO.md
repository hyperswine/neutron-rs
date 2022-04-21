# TODO

- write to uart doesnt seem to be working on qemu system aarch64. SIGTRAP. Maybe didnt setup stack properly at a specific v addr. Gonna just use a linker script and see the ELF output because it looks right
- make cargo test work on riscv and arm with a qemu runner. Phils got it done properly
- maybe requires xtask to do it properly. Otherwise I guess `arcutils test --elf-img build/neutron_kernel` could work

## Problem: SIGTRAP

SIGTRAP upon getting to `_start`. On the lldb remote debugger.
I have no idea what is going on. Seems to be some issue with QEMU getting to the entry??
I dont know if it even loaded the ELF file properly.

I dunno how to analyse QEMU runs. Maybe check the exit code. UPDATE: it is `1`. So that means SIGTRAP wasnt forwarded right and QEMU just `exit 1`.

I dunno how gdb analyse symbols work. I wanna print the elf headers and symtab.
