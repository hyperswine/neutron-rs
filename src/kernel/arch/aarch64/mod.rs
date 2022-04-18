pub mod memory;

#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x10000000 as *mut u8;
        let _bytes = $exact.bytes();
        for byte in _bytes {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write(p, byte),
                    _ => core::ptr::write(p, 0xfe),
                }
            }
        }
    };
}
