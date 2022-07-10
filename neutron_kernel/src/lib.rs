#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![allow(named_asm_labels)]
#![feature(asm_const)]
#![feature(step_trait)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]

// -----------------------
// CRATE WIDE API
// -----------------------

// NOTE: for tests, just use extern crate alloc and link to the hosts' alloc
extern crate alloc;
extern crate goblin;

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

pub mod drivers;
// * for now, dont bother
// pub mod exception;
pub mod filesystem;
pub mod kmod;
pub mod process;
pub mod services;
pub mod types;
// ALLOCATOR AND THE REST
pub mod memory;
// Kernel Manager
pub mod kernel;

// -----------------------
// ARCH DEPENDENT CODE
// -----------------------

pub mod arch;
