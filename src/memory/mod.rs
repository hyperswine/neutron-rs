// SHOULD NOT BE REQUIRED SINCE GLOBAL ALLOC HANDLER DEFINED WILL USUALLY BE FINE
// BUT VM (virtual memory/paging) stuff may require

// #[cfg(target_arch = "riscv64")]
// #[path = "../kernel/arch/riscv64gc/mod.rs"]
// pub mod riscv64gc;

// #[cfg(target_arch = "aarch64")]
// #[path = "../kernel/arch/aarch64/mod.rs"]
// pub mod aarch64;

// TODO: expose alloc and paging functions to the interrupt module
pub mod interrupt;
