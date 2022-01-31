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
        print!("Works!");
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
