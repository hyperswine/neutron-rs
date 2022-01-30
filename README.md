# Neutron
A rust based kernel built on first principles.

# Testing
A core idea is TDD. The only way to really verify that things work. Each `mod.rs` should contain cfg() for build and test, esp for arch dependent code. For arch independent code, should try not to rely on arch dependent code. I.e. only arch dependent code should rely on arch independent code. Or the interface should be isolated, i.e. in kernel manager.

## Arctest
Kinda hacky to use the builtin `cargo test` and no guarentee about its reliability.
So:
- could build `arcboot test` as a way to enable stuff with `cfg(arctest)`, e.g. build with `--cfg 'feature arctest'`. Then link the kernel as a library to find the functions with `cfg(arctest)` and get their function pointers.
- UNIT TESTING Option 1: scan through the files one by one with the .rs extension. Find functions with `cfg(arctest)` and store them somehow. Maybe store their paths or somehow add their function pointers to a global list. Maybe that `#[cfg(feature = arctest)]` also adds a macro to the function that exports its function pointer to lib.rs. Then you can build and run it as a test config. Or literally copy and paste the functions into arctest.rs and somehow manage the imports -> prob just go with this for now.
Not very robust and prob has a bunch of edge issues. But simple and works.
- Option 2: go through and see where all the test functions are. Then import them into kernel/arch/mod.rs at the top. Then create a `run_test` function at the bottom and call the functions sequentially. If a function passes without error, print (---- [OK]) to console
- INTEGRATION TESTING: add the functions to the global list and execute them one by one. Prob easier since they are in `tests/`.

# Dependencies
Rust (rustup recommended)
 - rust-src
 - target aarch64 and riscv64 (unknown-none)
QEMU
Toolchains (add to path or specify sysroot when using `cargo build`)
 - aarch64-gcc
 - riscv64-gcc

# Building
`cargo build`

# Running
`run.sh`
