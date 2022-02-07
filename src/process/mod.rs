// pub mod manager;

pub struct Process;

pub enum ProcessPrivilege {
    FULL, RD_ONLY, RD_WRITE, NONE
}

// -------------
// CARGO TEST
// -------------

#[test]
fn test_process() {
    let process = Process{};
    assert_eq!(1, 1);
    println!("process succesfully created!");
}

// -------------
// ARCTEST
// -------------

// WHEN TRYING TO TEST DRIVER AND OUTPUT, USE THE HIGHER LEVEL FUNCTIONS AND CHECK OUTPUT DIRECTLY
// DO NOT TEST THE FOLLOWING IN CARGO-BASED UNIT TESTS (SINCE THEY RUN ON THE HOST INSTEAD OF THE VM)

// IF TRYING TO TEST ON THE VM, Use arcboot test instead
 // Cargo is supposed to only test functions and function interfaces
#[cfg(feature = "arctest")]
fn test_serial_out_basics() {
    use core::ptr;
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"In test_process(), successfully created a process\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
