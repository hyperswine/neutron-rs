// ---------------
// COMMON
// ---------------

use core::{arch::global_asm, cell::UnsafeCell, fmt};
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

use crate::{
    exception::IRQContext,
    kernel::{final_setup, PrivilegeLevel},
    println, write_uart,
};

// ---------------
// PRIVILEGE LEVEL
// ---------------

#[no_mangle]
pub unsafe extern "C" fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
    prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr);
    cortex_a::asm::eret()
}

#[inline(always)]
unsafe fn prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    CNTVOFF_EL2.set(0);
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    ELR_EL2.set(kernel_init as *const () as u64);
    SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
}

// -------------
// SETUP
// -------------

unsafe fn kernel_init() -> ! {
    // INITIALISE DEVICE DRIVERS
    use core::time::Duration;

    println!(
        "{} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let (_, privilege_level) = current_privilege_level();
    println!("Current privilege level: {}", privilege_level);

    write_uart!(b"Exception handling state:");

    // Transition to common code in kernel
    final_setup()
}

// -------------
// EXCEPTIONS
// -------------

/// Wrapper structs for memory copies of registers.
#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);
struct EsrEL1(InMemoryRegister<u64, ESR_EL1::Register>);

/// The exception context as it is stored on the stack on exception entry.
#[repr(C)]
struct ExceptionContext {
    gpr: [u64; 30],
    lr: u64,
    elr_el1: u64,
    spsr_el1: SpsrEL1,
    esr_el1: EsrEL1,
}

/// Prints verbose write_uartrmation about the exception and then panics.
/// RN, does nothing basically. Otherwise check CSR of exception numbers and etc. And call the specified handler for that exception
fn default_exception_handler(exc: &ExceptionContext) {
    panic!("CPU Exception!\n");
}

//------------------
// Current, EL0
//------------------

#[no_mangle]
unsafe extern "C" fn current_el0_synchronous(_e: &mut ExceptionContext) {
    panic!("Should not be here. Use of SP_EL0 in EL1 is not supported.")
}

#[no_mangle]
unsafe extern "C" fn current_el0_irq(_e: &mut ExceptionContext) {
    panic!("Should not be here. Use of SP_EL0 in EL1 is not supported.")
}

#[no_mangle]
unsafe extern "C" fn current_el0_serror(_e: &mut ExceptionContext) {
    panic!("Should not be here. Use of SP_EL0 in EL1 is not supported.")
}

//-------------------
// Current, ELx
//-------------------

#[no_mangle]
unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext) {
    #[cfg(feature = "test_build")]
    {
        const TEST_SVC_ID: u64 = 0x1337;

        if let Some(ESR_EL1::EC::Value::SVC64) = e.esr_el1.exception_class() {
            if e.esr_el1.iss() == TEST_SVC_ID {
                return;
            }
        }
    }

    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(_e: &mut ExceptionContext) {
    let token = &IRQContext::new();
    // USING PI 4 drivers for this??
    // crate::drivers::pi4b::exception::irq_manager().handle_pending_irqs(token);
}

#[no_mangle]
unsafe extern "C" fn current_elx_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//-----------------
// Lower, AArch64
//-----------------

#[no_mangle]
unsafe extern "C" fn lower_aarch64_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------
// Lower, AArch32
//------------------

// may be needed for aarch64 since there are shared instructions

#[no_mangle]
unsafe extern "C" fn lower_aarch32_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------
// MORE EXCEPTIONS
//------------------

impl EsrEL1 {
    #[inline(always)]
    fn exception_class(&self) -> Option<ESR_EL1::EC::Value> {
        self.0.read_as_enum(ESR_EL1::EC)
    }

    #[cfg(feature = "test_build")]
    #[inline(always)]
    fn iss(&self) -> u64 {
        self.0.read(ESR_EL1::ISS)
    }
}

/// Human readable ESR_EL1.
#[rustfmt::skip]
impl fmt::Display for EsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ESR_EL1: {:#010x}", self.0.get())?;
        write!(f, "Exception Class (EC): {:#x}", self.0.read(ESR_EL1::EC))?;

        let ec_translation = match self.exception_class() {
            Some(ESR_EL1::EC::Value::DataAbortCurrentEL) => "Data Abort, current EL",
            _ => "N/A",
        };

        writeln!(f, " - {}", ec_translation)?;
        write!(f, "Instr Specific Syndrome (ISS): {:#x}", self.0.read(ESR_EL1::ISS))
    }
}

impl ExceptionContext {
    #[inline(always)]
    fn exception_class(&self) -> Option<ESR_EL1::EC::Value> {
        self.esr_el1.exception_class()
    }

    #[inline(always)]
    fn fault_address_valid(&self) -> bool {
        use ESR_EL1::EC::Value::*;

        match self.exception_class() {
            None => false,
            Some(ec) => matches!(
                ec,
                InstrAbortLowerEL
                    | InstrAbortCurrentEL
                    | PCAlignmentFault
                    | DataAbortLowerEL
                    | DataAbortCurrentEL
                    | WatchpointLowerEL
                    | WatchpointCurrentEL
            ),
        }
    }
}

/// The processing core/thread's current privilege level.
pub fn current_privilege_level() -> (PrivilegeLevel, &'static str) {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor, "EL2"),
        Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel, "EL1"),
        Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User, "EL0"),
        _ => (PrivilegeLevel::Unknown, "Unknown"),
    }
}

/// Init exception handling by setting the exception vector base address register.
pub unsafe fn handling_init() {
    // Provided by exception.S
    extern "Rust" {
        static __exception_vector_start: UnsafeCell<()>;
    }

    VBAR_EL1.set(__exception_vector_start.get() as u64);

    // Force VBAR update to complete before next instruction.
    barrier::isb(barrier::SY);
}
