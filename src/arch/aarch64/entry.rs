// ---------------
// COMMON
// ---------------

use crate::arch::aarch64::exception::init_interrupt_handlers;
use arcboot_api::ArcServices;
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

// if page is unmapped flagged (page descriptor not there in L0,1,2) or just all 0s
// ARM should pass the exception syndrome = page fault and what vaddr it faulted at or entry it faulted at
pub fn page_fault_handler() {
    // 6 bits telling us what kind of fault

    // if all 0s, would prob be UNKNOWN descriptor fault
    // then just get a new frame for the vaddr-page
    // if a stack access, maybe allocate 2
    // if a heap access, maybe allocate 16
}

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
