pub mod manager;

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

