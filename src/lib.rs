#![no_std]
#![feature(alloc_error_handler)]
#![allow(named_asm_labels)]
#![feature(global_asm)]
#![feature(asm_const)]
#![feature(step_trait)]
#![feature(trait_alias)]
#![feature(core_intrinsics)]

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

pub mod arch;
pub mod drivers;
pub mod exception;
pub mod filesystem;
pub mod kmod;
pub mod process;
pub mod services;
pub mod types;
// ALLOC AND THE REST
pub mod memory;

// -----------------------
// ARCH DEPENDENT CODE
// -----------------------

// NOTE: for tests, just use extern crate alloc and link to the hosts' alloc
extern crate alloc;
extern crate goblin;

use core::{fmt, panic::PanicInfo};

// Kernel Manager and ARCH Specific
pub mod kernel;
