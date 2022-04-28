pub mod manager;
pub mod sparc;
pub mod syscall;

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

// WRITE TO DISPLAY BUFFER (VGA MONITOR DRIVER)
// https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/uart.c great resource, system v unix reconstruction in risc v
// For hardware interrupts, https://github.com/riscv/riscv-aclint

// -------------
// INTERFACE
// -------------

// use UNIX like constructs for file descriptors (Descriptor)

type Descriptor = u64;

type FileDescriptor = Descriptor;
type SocketDescriptor = Descriptor;

pub enum ErrNo {
    READ_VALID,
}

struct ServiceStatus {
    errno: ErrNo,
    status: i8,
}

impl ServiceStatus {
    pub fn new() -> Self {
        Self {
            errno: ErrNo::READ_VALID,
            status: 1,
        }
    }
}

// SYSCALL HANDLERS SHOULD CALL THESE FUNCTIONS

fn mount(dev_id: u64, filepath: &str) -> ServiceStatus {
    ServiceStatus::new()
}

fn dismount(filepath: &str) -> ServiceStatus {
    ServiceStatus::new()
}

// ---------
// TEST
// ---------

use crate::write_uart;

#[test_case]
fn test_basic_mounting() {
    // TODO: represent a device in drivers. A driver should correspond to a device type
    // A device type can be either generic (GenericDevice) or specific (DeviceSpec)
    struct Device;
    // create a device
    let device = Device;
    mount(0, "/dev/default");
    write_uart!(b"Mounted succesfully!\n");
}
