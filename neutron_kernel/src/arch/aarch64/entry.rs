// ---------------
// COMMON
// ---------------

use crate::arch::aarch64::exception::init_interrupt_handlers;
use core::{cell::UnsafeCell, fmt};
use arcboot_api::ArcServices;
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
// maybe a field interrupt_table_start or vbar_start (virtually mapped into kernel's addr space)

/// Call this right after arc entry
pub fn arch_init(arcservices: ArcServices) {
    info!("Entry into arch! Arch: AARCH64");

    // setup interrupts and handlers
    unsafe {
        init_interrupt_handlers(0x80000);
    }

    // Return & transition to common code in kernel. Will need kernel.rs after all
}

#[test]
fn test_entry() {
    // DONT TRY THIS WITHOUT A VM!
    // arch_init();
    println!("It works!")
}
