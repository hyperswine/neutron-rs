#[cfg(target_arch = "riscv")]
pub mod riscv64gc;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

// FOR INTEGRATION TESTING, use specify a testing profile, e.g. test_rv or test_aarch
// FOR UNIT TESTING, run it for all the modules, i.e. just put the test function in the arch dependent modules and run the cargo test suite on any host
