#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![allow(named_asm_labels)]
#![feature(asm_const)]
#![feature(step_trait)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

use core::alloc::Layout;

extern crate alloc;
extern crate goblin;
extern crate log;

pub mod drivers;
pub mod exception;
pub mod filesystem;
pub mod process;
pub mod services;
pub mod memory;
pub mod userspace;
pub mod time;
// ARCH DEPENDENT CODE
pub mod arch;

#[cfg(no_std)]
#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
