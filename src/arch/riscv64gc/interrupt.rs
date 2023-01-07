// HANDLERS FOR SERVICE-RELATED INTERRUPTS
// SYSCALLS
// FOR PAGING, see memory/interrupt.rs

// Up to 4096 available CSRs in the CS address space
// can define our own registers for optimisation

const SUPERVISOR_INTERRUPT_ENABLE: u64 = 0x0104;
const SUPERVISOR_TRAP_BASE_ADDR: u64 = 0x0105;
const SUPERVISOR_INTERRUPT_CAUSE: u64 = 0x0142;

struct IDT;

use crate::alloc::{vec, vec::Vec};

// define interrupt handler function type
type InterruptHandlerType = fn(i32, i32) -> i32;

// Import all the syscall functions (or just NeutronSyscall struct that implements them)
// use crate::services::syscall::*;

// initialise a lazy static vector
// lazy_static!{
//     static ref INTERRUPT_DESCRIPTOR_TABLE: Vec<InterruptHandlerType> = Vec::new(
        // syscall 0 -> open
        // NeutronService::open
//     );
// }

// INITIALISE INTERRUPTS (after Kernel Entry completed and userspace ready to load)
// interrupts should be disabled by bios during boot
fn initialize_interrupts() {
    // set control/status reg for enabling interrupts
    // write_reg!(SUPERVISOR_INTERRUPT_ENABLE, 1);

    // create IDT at the same frame addr
    // NOTE: 2^32 possible interrupt types, stored within scause
    // Neutron has ~20 key syscalls, page fault handling, double fault handling
    // triple faults automatically handled by resetting the system/power off -> on
    // let frames = get_frames(enough to store n_interrupts)
    // let idt = IDT::new(frames)
}

// Need to register this function in the IDT
fn syscall_interrupt_handler() {
    
}
