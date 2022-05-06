// ---------------
// COMMON
// ---------------

use core::{arch::global_asm, cell::UnsafeCell, fmt};
use cortex_a::{asm::barrier, registers::*};
use tock_registers::{
    interfaces::{Readable, Writeable},
    registers::InMemoryRegister,
};

use crate::kernel::final_setup;

// ---------------
// PRIVILEGE LEVEL
// ---------------

// Use ERET to go to a lower execution level. Requires extra code
#[no_mangle]
pub unsafe extern "C" fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
    prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr);

    // Use `eret` to "return" to EL1. This results in execution of kernel_init() in EL1.
    cortex_a::asm::eret()
}

#[inline(always)]
unsafe fn prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
    // Enable timer counter registers for EL1.
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters.
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64.
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return to go back to EL1
    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );

    // Second, let the link register point to kernel_init().
    ELR_EL2.set(kernel_init as *const () as u64);

    // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there are no plans to ever return to EL2, just re-use the same stack.
    SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
}

// -------------
// SETUP
// -------------

unsafe fn kernel_init() -> ! {
    // use driver::interface::DriverManager;

    // for i in bsp::driver::driver_manager().all_device_drivers().iter() {
    //     if let Err(x) = i.init() {
    //         panic!("Error loading driver: {}: {}", i.compatible(), x);
    //     }
    // }
    // bsp::driver::driver_manager().post_device_driver_init();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    // use bsp::console::console;
    // use console::interface::All;
    // use core::time::Duration;
    // use driver::interface::DriverManager;
    // use time::interface::TimeManager;

    // info!(
    //     "{} version {}",
    //     env!("CARGO_PKG_NAME"),
    //     env!("CARGO_PKG_VERSION")
    // );
    // info!("Booting on: {}", bsp::board_name());

    // let (_, privilege_level) = exception::current_privilege_level();
    // info!("Current privilege level: {}", privilege_level);

    // info!("Exception handling state:");
    // exception::asynchronous::print_state();

    // info!(
    //     "Architectural timer resolution: {} ns",
    //     time::time_manager().resolution().as_nanos()
    // );

    // info!("Drivers loaded:");
    // for (i, driver) in bsp::driver::driver_manager()
    //     .all_device_drivers()
    //     .iter()
    //     .enumerate()
    // {
    //     info!("      {}. {}", i + 1, driver.compatible());
    // }

    // info!("Timer test, spinning for 1 second");
    // time::time_manager().spin_for(Duration::from_secs(1));
    
    // Transition to common code in kernel
    final_setup()
}

// * Ensure this is included
core::arch::global_asm!(include_str!("meta.s"));
core::arch::global_asm!(include_str!("entry.s"));

// -------------
// EXCEPTIONS
// -------------

// TODO: exceptions
