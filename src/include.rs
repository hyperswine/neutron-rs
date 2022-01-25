// FOR COMMON INCLUDES FOR BUILD
// no test, for test use test mod

#[cfg(any(target_arch = "riscv64", target_arch = "aarch64"))]
extern crate alloc;
#[cfg(any(target_arch = "riscv64", target_arch = "aarch64"))]
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};
