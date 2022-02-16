pub mod manager;
pub mod sparc;

pub struct MemoryBlock {
    mem: *mut u8,
    size_bytes: u64,
}

pub trait Service {
    // request this service from the kernel
    fn request();
    // receive the result in the form of a readable buffer/memory block/pages
    fn receive() -> MemoryBlock;
}

// WRITE TO DISPLAY BUFFER (VGA MONITOR DRIVER)
// Need -display gtk (for a framebuffer I think or mostly controlling app level display options)
// Maybe -vga cirrus or -vga std or -vga virtio

// For hardware interrupts, https://github.com/riscv/riscv-aclint
