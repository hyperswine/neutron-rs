pub mod memory;
pub mod power;

static HELLO: &[u8] = b"Hello World!";

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let virtio_framebuffer = 0x100081ff as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *virtio_framebuffer.offset(i as isize * 2) = byte;
            *virtio_framebuffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
