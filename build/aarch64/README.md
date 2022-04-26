# Link

When building, link with multiboot.x script + multiboot_entry.o

Later, I'll prob make a build.rs or xtask/arcutils to build either for entry or non entry. And get rid of those things in config.toml.

## Build

If entry.o and multiboot_entry.o doesnt exist, you need to run:

```bash
aarch64-elf-as src/kernel/arch/aarch64/entry/multiboot_entry.s build/aarch64/multiboot.o
```
