/*#![no_std]
#![no_main]

use neutron_kernel::{write_uart};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    write_uart!("[test did not panic]");
    loop {}
}

fn should_fail() {
    write_uart!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    write_uart!("[ok]");
    loop {}
}*/