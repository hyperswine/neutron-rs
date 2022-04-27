pub mod memory;

// -----------------
// BASIC UART OUTPUT
// -----------------

#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x09000000 as *mut u8;
        for byte in $exact {
            unsafe {
                // match byte {
                //     0x20..=0x7e | b'\n' => core::ptr::write_volatile(p, *byte),
                //     _ => core::ptr::write_volatile(p, 0xfe),
                // }
                core::ptr::write_volatile(p, *byte);
            }
        }
    };
}

static GREETING: &[u8] = b"Hello World!\n";

pub fn print_uart0(bytes: &[u8]) {
    let p = 0x09000000 as *mut u8;
    for byte in bytes {
        unsafe {
            core::ptr::write_volatile(p, *byte);
        }
    }
}

pub fn display_greeting() {
    print_uart0(GREETING);
}

// -------------
// SETUP
// -------------

use core::arch::{asm};

// KEY FUNCTION. MUST LOAD RIGHT AFTER _start to set the right registers and confirm paging
pub fn _load() {
    unsafe {
        // SET STACK POINTER
        asm!(
            "
            ldr x30, =stack_top
            mov sp, x30
            "
        );

        // MULTIBOOT HEADER
        asm!(
            "
            .section .multiboot_header
            header_start:
                .quad 0xe85250d6
                .quad 0
                .quad header_end - header_start
                .quad 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
                .word 0
                .word 0
                .quad 8
            header_end: 
            "
        );
    }

    use cortex_a::registers::*;
    use tock_registers::interfaces::Writeable;

    // BOOT CORES from https://docs.rs/crate/cortex-a/2.5.0
    const CORE_MASK: u64 = 0x3;

    // GO INTO EL2 (from EL1)
    CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

    // No offset for reading the counters
    CNTVOFF_EL2.set(0);

    // Set EL1 execution state to AArch64
    HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

    // Set up a simulated exception return
    // __exception_return();
}

fn __exception_return() {
    use cortex_a::registers::SPSR_EL2;
    use tock_registers::interfaces::Writeable;

    SPSR_EL2.write(
        SPSR_EL2::D::Masked
            + SPSR_EL2::A::Masked
            + SPSR_EL2::I::Masked
            + SPSR_EL2::F::Masked
            + SPSR_EL2::M::EL1h,
    );
}

#[cfg(target_arch = "aarch64")]
pub fn basic_greet() {
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
