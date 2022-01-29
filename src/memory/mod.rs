#[cfg(target_arch = "riscv64")]
#[path = "../kernel/arch/riscv64gc/mod.rs"]
pub mod riscv64gc;

#[cfg(target_arch = "aarch64")]
#[path = "../kernel/arch/aarch64/mod.rs"]
pub mod aarch64;
