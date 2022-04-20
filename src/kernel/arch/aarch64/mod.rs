pub mod memory;

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
