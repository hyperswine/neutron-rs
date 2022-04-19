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
