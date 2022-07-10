// ---------------
// COMMON
// ---------------

use crate::userland::PrivilegeLevel;
use core::{cell::UnsafeCell, fmt};
use cortex_a::{asm::barrier, registers::*};
use log::info;
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

// -------------
// SETUP
// -------------

// ? maybe pass the arc memory map here

/// Call this right after arc entry
fn arch_init() {
    info!("Entry into arch! Arch: AARCH64");

    // setup interrupts and handlers
    unsafe {
        init_interrupt_handlers(0x80000);
    }

    // Transition to common code in kernel. Maybe need kernel.rs after all
    // final_setup();
}

#[test]
fn test_entry() {
    // DONT TRY THIS WITHOUT A VM!
    // arch_init();
    println!("It works!")
}
