// UNIT TESTS

#[macro_use]
use crate::print;

// TYPES
// call this from run_tests() in riscv64. Use that to call other test functions
pub fn test_types() {
    // ? if there was a way to introspect and collect the function pointers here
    // for test in tests {
    //     test();
    // }

    fn test_bytes() {
        let byte = crate::types::Bytes::from_int(50);
        // print!("{}", byte.content[0]);
        // print!("Works!");
        use core::ptr;
        const UART0: *mut u8 = 0x10000000 as *mut u8;
        let out_str = b"succesfully done\n";
        for byte in out_str {
            unsafe {
                // maybe already filled
                ptr::write_volatile(UART0, *byte);
            }
        }
    }

    test_bytes();
}

// PROCESSES

pub fn test_processes() {
    fn test_process_basic() {
        let process = crate::process::Process;
        // println!("created a process!");
    }
}

// FILESYSTEM
