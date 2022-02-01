// Write to MMIO

unsafe fn mmio_write(address: usize, offset: usize, value: u8) {
	let reg = address as *mut u8;
	reg.add(offset).write_volatile(value);
}

unsafe fn mmio_read(address: usize, offset: usize, value: u8) -> u8 {
	let reg = address as *mut u8;
	reg.add(offset).read_volatile()
}

// RISC-V QEMU (spike) MMIO in C++
// static struct MemmapEntry {
// 	hwaddr base;
// 	hwaddr size;
// } virt_memmap[] = {
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
// };

pub fn uart_init(base_addr: usize) {
	let ptr = base_addr as *mut u8;
	unsafe {
		let lcr = (1 << 0) | (1 << 1);
		ptr.add(3).write_volatile(lcr);
		ptr.add(2).write_volatile(1 << 0);
		ptr.add(1).write_volatile(1 << 0);

		let divisor: u8 = 592;
		let divisor_least: u8 = divisor & 0xff;
		let divisor_most:  u8 = divisor >> 8;

		ptr.add(3).write_volatile(lcr | 1 << 7);
		ptr.add(0).write_volatile(divisor_least);
		ptr.add(1).write_volatile(divisor_most);
		ptr.add(3).write_volatile(lcr);
	}
}

// fn uart_get(base_addr: usize) -> Option {
//     let ptr = base_addr as *mut u8;
//     unsafe {
//         if ptr.add(5).read_volatile() & 1 == 0 {
//             None
//         }
//         else {
//             Some(ptr.add(0).read_volatile())
//         }
//     }
// }

fn uart_put(base_addr: usize, c: u8) {
	let ptr = base_addr as *mut u8;
	unsafe {
		ptr.add(0).write_volatile(c);
	}
}

pub struct Uart {
	base_address: usize,
}

use core::fmt::{Error, Write};

impl Uart {
	pub fn new(base_address: usize) -> Self {
		Uart {
			base_address
		}
	}
}

impl Write for Uart {
	fn write_str(&mut self, s: &str) -> Result<(), Error> {
		for c in s.bytes() {
			uart_put(0x10000000, c);
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! print{
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!(crate::uart::UartDriver::new(0x1000_0000), $($args)+);
	});
}

#[macro_export]
macro_rules! println{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}
