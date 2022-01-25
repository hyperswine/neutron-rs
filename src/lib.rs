// only put high level configs for testing, riscv build, aarch64 build
// can use std for test
// cargo uses this to build stuff. So if you are testing on x864 it will include everything and look at cfg(test) stuff to include and stuff that isnt marked with another arch
// dont mark stuff with cfg(x86_64) unless you want to support it, which I dont. So just do cfg(test) for now

// UNIT TESTS
#[test]
fn trivial_assertion() {
    println!("checking compiler sanity");
    assert_eq!(1, 1);
    println!("nice, basics out of the way!");
}

#[test]
fn test_process() {
    let process = process::Process;
    println!("created a process!");
}

#[cfg(test)]
mod test;

// ARCH DEPENDENT CODE

// why isnt cfg(not(test)) working? https://github.com/rust-lang/rust/issues/59168 something to do with the fact that cargo test implementation runs it for the 'cfg(test) crate', not to any of its dependencies
// so that means it still looks at arch_manager since its not a dependency and wishes to compile it. [features] test = [] also apparently works out of the box for cfg(not(test))

// doesnt work. I tried with test = [] and it still doesnt work
// #[cfg(not(test))]
// pub mod arch_manager;

// workaround 1, though I think this is for positive instead of negative since I didnt define testing property
// #[cfg(any(test, feature = "testing"))]
// pub mod arch_manager;

// workaround 2, building for aarch64 or riscv (too bad x86_64 wont be included though)
// preferred workaround
#[cfg(any(target_arch = "riscv64", target_arch = "aarch64"))]
pub mod arch_manager;

// NON ARCH DEPENDENT CODE

// ! I dunno if this actually works. Does the compiler not use std maybe
#[cfg(any(target_arch = "riscv64", target_arch = "aarch64"))]
extern crate alloc;
#[cfg(any(target_arch = "riscv64", target_arch = "aarch64"))]
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};

pub mod process;
pub mod kernel;
pub mod types;
