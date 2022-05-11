// ARCH DEPENDENT CODE
// TODO: should be placed in src/
// and its stuff imported in src/memory, src/drivers, src/kernel, etc.

#[cfg(target_arch = "riscv64")]
pub mod riscv64gc;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86;
