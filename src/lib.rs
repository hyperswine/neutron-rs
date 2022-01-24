// only put high level configs for testing, riscv build, aarch64 build
// can use std for test
// cargo uses this to build stuff. So if you are testing on x864 it will include everything and look at cfg(test) stuff to include and stuff that isnt marked with another arch
// dont mark stuff with cfg(x86_64) unless you want to support it, which I dont. So just do cfg(test) for now

// TESTS
#[cfg(test)]
fn main() {
    println!("Starting tests");
    trivial_assertion()
}

#[test]
fn trivial_assertion() {
    println!("trivial assertion");
    assert_eq!(1, 1);
    println!("ok");
}

// maybe if config not test, can include lib.rs
// else include the non dependent arch code directly
// and test the dependent arch code within their modules with custom test runners
// and etc

// ! wait actually just do it in kernel and arch
// yea just do pub mod kernel here and let them handle it
pub mod kernel;

// do it in arch and drag to kernel
// #[cfg(target_arch = "riscv")]
// pub mod kernel{arch::riscv}; // use the riscv module from kernel 

// #[cfg(target_arch = "aarch64")]
// pub mod kernel::aarch64; // use the aarch64 module from kernel 

