pub mod interrupt;
pub mod manager;
pub mod sparc;
pub mod syscall;

pub struct MemoryBlock {
    mem: *mut u8,
    size_bytes: u64,
}

// ! USING SPARC INSTEAD
pub trait OldService {
    // request this service from the kernel
    fn request();
    // receive the result in the form of a readable buffer/memory block/pages
    fn receive() -> MemoryBlock;
}

// WRITE TO DISPLAY BUFFER (VGA MONITOR DRIVER)
// https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/uart.c great resource, system v unix reconstruction in risc v
// For hardware interrupts, https://github.com/riscv/riscv-aclint
