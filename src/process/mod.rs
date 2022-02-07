// pub mod manager;

pub struct Process;

pub enum ProcessPrivilege {
    FULL, RD_ONLY, RD_WRITE, NONE
}

#[test]
fn test_process() {
    let process = Process{};

    use core::ptr;
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"In test_process(), successfully created a process\n";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }

    assert_eq!(1, 0);
}
