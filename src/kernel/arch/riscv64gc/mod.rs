pub mod memory;
pub mod power;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
