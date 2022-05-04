// IF CAN GET REEXPORT TEST HARNESS TO WORK WITH MODULES, THEN DONT NEED ARCTEST
// ALSO WORKS FOR INT TESTS IT SEEMS

#[cfg(feature = "arctest")]
fn test_service() {
    // test reading from and to disk (virtual)
    let service = ServiceManager;
    // create a virtual harddisk and mount, auto mounted at /mnt/vhdi, read writable by default user
    let vhd = tools::create_mount_vhd(disk_number, n_bytes, services::READ_WRITE);
    // ensure the drivers for virtual disk are working
    // maybe dont need since we can just treat it as a direct filesystem? rather than a device filesystem

    // blocking IO => have to implement async?
    let result = services::read_service(disk_number, offset, bytes);

    // println!("result = {}", result);
}

// -------------
// SAFE IO MEMORY
// -------------

// * prob not needed. Just use standard read/write services instead

// safe reading and writing of memory blocks
// for heapalloc/mmap like syscalls
pub struct MemoryBlock {
    mem: *mut u8,
    size_bytes: u64,
}

impl MemoryBlock {
    // given a pointer to the block of memory (dynamic/static) and the size of that block
    // create a mem block
    pub fn new(pointer: *mut u8, size_bytes: u64) -> Self {
        Self {
            mem: pointer,
            size_bytes,
        }
    }

    // return another memory block from the read memory
    // from a file on disk
    pub fn read() -> MemoryBlock {
        MemoryBlock {
            mem: todo!(),
            size_bytes: todo!(),
        }
    }

    // file path, block of memory to write, offset
    pub fn write(memblock: MemoryBlock) {}
}
