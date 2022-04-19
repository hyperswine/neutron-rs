// -------------------
// MALI (MIDGARD v6) GPU DRIVERS
// -------------------

use core::mem::size_of;

use alloc::string::String;

// for 64 bits, usize == u64
type MMIOAddrRange = (usize, usize);

// TODO: could prob read in the spec list from hardware/rk3399.yml
// And create generic graphics, cpu, etc. devices for each and assign them a name
pub struct MaliT880 {
    mmio_address_range: MMIOAddrRange,
    command_submit_addr: usize,
}

// a pointer to a block of data in physical memory
// in userspace
pub type CommandBuffer = usize;

impl MaliT880 {
    // userspace -> kernelspace mmio
    pub fn submit_command_buffer(&self, size: usize, command_buffer: CommandBuffer) {
        // copy the command buffer to the specified address
        unsafe {
            for i in 0..size {
                // TODO: this is actually writing 64bits at a time, we should be writing byte by byte
                core::ptr::write_volatile(self.command_submit_addr as *mut usize, *((command_buffer+i*size_of::<usize>()) as *mut usize));
            }
        }
    }
}
