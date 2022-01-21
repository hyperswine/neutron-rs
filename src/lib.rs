#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;

mod vga_buffer;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/*
    Entry point for the Kernel
*/

#[no_mangle]
pub extern "C" fn _start() -> ! {
    const UART0: *mut u8 = 0x10000000 as *mut u8;
    let out_str = b"riscv64 bare metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
    // VGA
    println!("Hello World{}", "!");
    loop {}
}

/*
    Setup interfaces for filesystem, memory, networking, etc
*/
// static struct MemmapEntry {
// 	hwaddr base;
// 	hwaddr size;
//     virt_memmap[] = {
// 	[VIRT_DEBUG] =       {        0x0,         0x100 },
// 	[VIRT_MROM] =        {     0x1000,       0x11000 },
// 	[VIRT_TEST] =        {   0x100000,        0x1000 },
// 	[VIRT_CLINT] =       {  0x2000000,       0x10000 },
// 	[VIRT_PLIC] =        {  0xc000000,     0x4000000 },
// 	[VIRT_UART0] =       { 0x10000000,         0x100 },
// 	[VIRT_VIRTIO] =      { 0x10001000,        0x1000 },
// 	[VIRT_DRAM] =        { 0x80000000,           0x0 },
// 	[VIRT_PCIE_MMIO] =   { 0x40000000,    0x40000000 },
// 	[VIRT_PCIE_PIO] =    { 0x03000000,    0x00010000 },
// 	[VIRT_PCIE_ECAM] =   { 0x30000000,    0x10000000 },
//     };
// }
