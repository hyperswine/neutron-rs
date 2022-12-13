use limine::*;

static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);
static MMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);

#[no_mangle]
extern "C" fn limine_main() -> ! {

    let bootloader_info = BOOTLOADER_INFO
        .get_response()
        .get()
        .expect("barebones: recieved no bootloader info");

    let mmap = MMAP
        .get_response()
        .get()
        .expect("barebones: recieved no mmap")
        .memmap();

    loop {}
}
