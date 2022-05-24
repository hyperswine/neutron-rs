use stivale_boot::{stivale2hdr, v2::*};

use crate::_common;

const STACK_SIZE: usize = 4096 * 16;

#[repr(C, align(4096))]
struct P2Align12<T>(T);
static STACK: P2Align12<[u8; STACK_SIZE]> = P2Align12([0; STACK_SIZE]);

static STIVALE_TERM: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();
static STIVALE_FB: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&STIVALE_TERM as *const StivaleTerminalHeaderTag).cast());

#[stivale2hdr]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK.0.as_ptr_range().end)
    .tags((&STIVALE_FB as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
extern "C" fn limine_entry_point(boot_info: &'static StivaleStruct) -> ! {
    boot_info.terminal().unwrap().term_write()("Hello, world!");

    _common();

    loop {}
}
