// arch dependent kernel code
// for arch or device dependent code, e.g. boot, drivers -> use src/drivers or support/arch

#[cfg(target_arch = "riscv64")]
pub mod riscv64gc;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
