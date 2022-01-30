pub mod memory;
pub mod power;

use core::ptr;

// Entry point for the Kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"succesfully loaded _start() on bare metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }

    // hook onto the start function to when testing, else ignore when building the final code
    #[cfg(feature = "arctest")]
    test_main();

    // exit after testing
    // #[cfg(test)]
    // exit(0);

    // create kernel
    // let kern_manager = kernel::KernelManager::new();
    // CALL kernel_main()

    // call clean_up() to write all pending operations to disk

    // loop for now so the function wont return (later can make it 'return' to bare metal aka exit/stop execution completely without an error code)
    loop {}
}

fn test_main() {

}
