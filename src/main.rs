#![no_main]
#![no_std]
#![feature(alloc_error_handler)]
// SUPPRESS WARNINGS
#![allow(dead_code)]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(neutron_kernel::test_runner)]

// -----------------------
// NON ARCH DEPENDENT CODE
// -----------------------

// required for main.rs
use core::panic::PanicInfo;
use neutron_kernel::write_uart;

// If running the test config directly, use test_panic_handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    neutron_kernel::test_panic_handler(info)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
#[cfg(not(target_arch = "riscv64"))]
extern "C" fn _start() -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        unsafe {
            use cortex_a::{asm, registers::*};
            use tock_registers::interfaces::Readable;
            use tock_registers::interfaces::Writeable;

            // BOOT CORES from https://docs.rs/crate/cortex-a/2.5.0
            const CORE_MASK: u64 = 0x3;
            const STACK_START: u64 = 0x7fff_ffff_0000_0000;
            // usually 2x page size, grows down infinitely until "hole"/47bit region
            const PER_THREAD_STACK_SIZE: u64 = 8192;
            const PHYSICAL_STACK_START: u64 = 0x40_000_000;

            match MPIDR_EL1.get() & CORE_MASK {
                0 => {
                    SP.set(STACK_START);
                }
                _ => loop {
                    // if not core0, infinitely wait for events
                    asm::wfe();
                },
            }

            // GO INTO EL2 (from EL1)
            CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

            // No offset for reading the counters.
            CNTVOFF_EL2.set(0);

            // Set EL1 execution state to AArch64.
            HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

            // Set up a simulated exception return.
            // SPSR_EL2.write(
            //     SPSR_EL2::D::Masked
            //         + SPSR_EL2::A::Masked
            //         + SPSR_EL2::I::Masked
            //         + SPSR_EL2::F::Masked
            //         + SPSR_EL2::M::EL1h,
            // );
        }
    }

    basic_greet();

    loop {}
}

#[cfg(feature = "multiboot")]
#[no_mangle]
extern "C" fn _multiboot_entry() -> ! {
    loop {}
}

#[cfg(target_arch = "aarch64")]
fn basic_greet() {
    use neutron_kernel::kernel::arch::aarch64::print_uart0;

    write_uart!(b"Hello World!\n");
    write_uart!(b"Hello World!\n");

    let p = 0x09000000 as *mut u8;
    for byte in b"Hi!" {
        unsafe {
            core::ptr::write_volatile(p, *byte);
        }
    }

    unsafe {
        core::ptr::write_volatile(p, b"H"[0]);
    }

    print_uart0(b"Hello, World!");

    loop {}
}
