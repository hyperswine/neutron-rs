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
extern "C" fn _start() -> ! {
    #[cfg(target_arch = "aarch64")]
    {
        neutron_kernel::kernel::arch::aarch64::_load();
        neutron_kernel::kernel::arch::aarch64::basic_greet();
    }

    loop {}
}
