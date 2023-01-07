use aarch64::regs::MPIDR_EL1;
use core::arch::asm;
use cortex_a::{registers::*, asm::barrier};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

// -------------
// EXCEPTIONS
// -------------

#[derive(PartialEq)]
pub enum PrivilegeLevel {
    Machine,
    User,
    Kernel,
    Hypervisor,
    Unknown,
}

/// Wrapper structs for memory copies of registers
#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);
struct EsrEL1(InMemoryRegister<u64, ESR_EL1::Register>);

/// The exception context is stored on the stack on exception entry
#[repr(C)]
struct ExceptionContext {
    gpr: [u64; 30],
    lr: u64,
    elr_el1: u64,
    spsr_el1: SpsrEL1,
    esr_el1: EsrEL1,
}

/// Prints info about the exception and then panics. For now
fn default_exception_handler(exc: &ExceptionContext) {
    panic!("CPU Exception!\n");
}

/// The processing core's current privilege level
pub fn current_privilege_level() -> PrivilegeLevel {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL3) => (PrivilegeLevel::Machine),
        Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor),
        Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel),
        Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User),
        _ => PrivilegeLevel::Unknown,
    }
}

// Maybe have an arcboot api that maps the interrupts properly to some memory hole
// Or set it up with some memory hole

/// Init exception handling by setting the exception vector base address register
/// Could do it manually or ask for it from the bootloader
pub unsafe fn init_interrupt_handlers(vector_table_start: u64) {
    VBAR_EL1.set(vector_table_start);

    barrier::isb(barrier::SY);
}

/// Return the executing core's id
// #[inline(always)]
// pub fn core_id<T>() -> T
// where
//     T: From<u8>,
// {
//     const CORE_MASK: u64 = 0b11;

//     T::from((MPIDR_EL1.get() & CORE_MASK) as u8)
// }

// -------------
// DAIF
// -------------

mod daif_bits {
    pub const IRQ: u8 = 0b0010;
}

trait DaifField {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register>;
}

// EXCEPTION TYPERS

struct Debug;
struct SError;
struct IRQ;
struct FIQ;

impl DaifField for Debug {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::D
    }
}

impl DaifField for SError {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::A
    }
}

impl DaifField for IRQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::I
    }
}

impl DaifField for FIQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::F
    }
}

fn is_masked<T>() -> bool
where
    T: DaifField,
{
    DAIF.is_set(T::daif_field())
}

pub fn is_local_irq_masked() -> bool {
    !is_masked::<IRQ>()
}

#[inline(always)]
pub unsafe fn local_irq_unmask() {
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn local_irq_mask() {
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline(always)]
pub unsafe fn local_irq_mask_save() -> u64 {
    let saved = DAIF.get();
    local_irq_mask();

    saved
}

#[inline(always)]
pub unsafe fn local_irq_restore(saved: u64) {
    DAIF.set(saved);
}
