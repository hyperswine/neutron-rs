pub mod entry;
pub mod memory;
pub mod syscall;

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

extern "C" fn print_uart0_c() {
    let p = 0x09000000 as *mut u8;
    for byte in b"Hello World!" {
        unsafe {
            core::ptr::write_volatile(p, *byte);
        }
    }
}

pub fn display_greeting() {
    print_uart0(GREETING);
}

// -------------
// FORMATTED OUTPUT
// -------------

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::kernel::arch::aarch64::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
    // ! call write uart. I think this panicked
    let a = args.as_str().unwrap();
    write_uart!(a.as_bytes());
}

// -------------
// SETUP
// -------------

use core::arch::asm;

// IDK how to ensure the labels are placed near the top
// I think we can maybe specify .multiboot_header = . + 0x10o or something

// TODO: modify this
// ensure this is included
core::arch::global_asm!(
    r#"
    .global _setup_stack
    _setup_stack:
        ldr x30, =stack_top
        mov sp, x30
        b _start
    
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
    "#
);

// TODO: implement this
pub fn kernel_init() {
    
}

#[cfg(target_arch = "aarch64")]
pub fn basic_greet() {
    write_uart!(b"Hello World!\n");
    print_uart0_c();

    loop {}
}
