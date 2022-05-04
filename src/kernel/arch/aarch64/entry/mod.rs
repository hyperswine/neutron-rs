// use this https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/09_privilege_level
// basically,

// core::arch::global_asm!(
//     "
// mrs	x0, CurrentEL
// cmp	x0, {CONST_CURRENTEL_EL2}
// b.ne	.L_parking_loop
// "
// );

// core::arch::global_asm!(
//     include_str!("init.s"),
//     CONST_CURRENTEL_EL2 = const 0x8,
//     CONST_CORE_ID_MASK = const 0b11
// );

// method 1: use ERET to go to a lower execution level. Requires extra code
#[no_mangle]
pub unsafe extern "C" fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
    prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr);

    // Use `eret` to "return" to EL1. This results in execution of kernel_init() in EL1.
    asm::eret()
}

use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;

#[inline(always)]
unsafe fn prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return.
    //
    // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
    // stack pointer.
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // TODO: in _start (aarch64/mod.rs), do this instead https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/09_privilege_level/src/_arch/aarch64/cpu/boot.s
    // then create a kernel_init
    // Second, let the link register point to kernel_init().
    ELR_EL2.set(super::kernel_init as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
    // are no plans to ever return to EL2, just re-use the same stack.
    SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
}
