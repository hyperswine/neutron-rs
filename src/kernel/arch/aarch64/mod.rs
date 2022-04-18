pub mod memory;

#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x09000000 as *mut u8;
        let _bytes = $exact.bytes();
        for byte in _bytes {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write_volatile(p, byte),
                    _ => core::ptr::write_volatile(p, 0xfe),
                }
            }
        }
    };
}

// extern "C" fn print_uart0() {
//     const char *s;
//     while(*s != '\0') {
//         *UART0DR = (unsigned int)(*s);
//         s++;	        
//     }
// }

#[cfg(not(test))]
#[no_mangle]
extern "C" fn _start() {
    loop {}
}
