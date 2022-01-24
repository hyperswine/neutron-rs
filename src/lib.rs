// only put high level configs for testing, riscv build, aarch64 build
// can use std for test
// cargo uses this to build stuff. So if you are testing on x864 it will include everything and look at cfg(test) stuff to include and stuff that isnt marked with another arch
// dont mark stuff with cfg(x86_64) unless you want to support it, which I dont. So just do cfg(test) for now

#[cfg(test)]
fn tester() {

}

// ! wait actually just do it in kernel and arch
// yea just do pub mod kernel here and let them handle it
pub mod kernel;

// do it in arch and drag to kernel
// #[cfg(target_arch = "riscv")]
// pub mod kernel{arch::riscv}; // use the riscv module from kernel 

// #[cfg(target_arch = "aarch64")]
// pub mod kernel::aarch64; // use the aarch64 module from kernel 

