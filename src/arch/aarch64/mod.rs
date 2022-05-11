pub mod entry;
pub mod exception;
pub mod translation;

// -----------------
// BASIC UART OUTPUT
// -----------------

#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x09000000 as *mut u8;
        for byte in $exact {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write_volatile(p, *byte),
                    _ => core::ptr::write_volatile(p, 0xfe),
                }
            }
        }
    };
}

// -----------------
// EXECUTING CORE
// -----------------

use aarch64::regs::MPIDR_EL1;
use tock_registers::interfaces::Readable;

/// Return the executing core's id.
#[inline(always)]
pub fn core_id<T>() -> T
where
    T: From<u8>,
{
    const CORE_MASK: u64 = 0b11;

    T::from((MPIDR_EL1.get() & CORE_MASK) as u8)
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

// I think this panicked, prob bad stack
pub fn _print(args: core::fmt::Arguments) {
    let a = args.as_str().unwrap();
    write_uart!(a.as_bytes());
}

#[cfg(target_arch = "aarch64")]
pub fn basic_greet() {
    crate::write_uart!(b"Hello World!\n");

    loop {}
}
