#[cfg(feature = "arctest")]
fn test_process() {
    // create a single process
    let process = Process{
        id: 1,
        name: "Dummy Process",
        permissions: 0, //ring 0, read, write and execute any part of the CPU and virtual memory and disk/filesystem
    };

    // try to allocate a whole bunch of memory as the process
    // should frantically increase the number of pages as the stack grows down. Also for the heap, so allocator should be called
    // args: n_bytes. res: mut* u8 to the block of memory. ProcessManager keeps track of the used memory (pages) for each Process 
    let ptr_to_mem = process.allocate_mem(500); // so when the program tries to access int i = 0; at stack address 0x60000000 it will go into the kernel which checks whether the program owns that page, if it does, it will allow that memory to be derefed at u32 (4B). if it doesnt own that memory and tries to write to it, rust should complain but not in unsafe mode
    // in unsafe mode, process can access whatever memory it wants so it should be the processmanager or page tables job to keep track of what memory they can access

    // try to access memory at address 0x60000000
    let res = process.access_addr(0x60000000, 500);
    std::assert!(check_optional(res));
    // try to illegally access an uninitialised byte far down the virtual addresses
    let res = process.access_addr(0x40000000, 1);
    std::assert!(!check_optional(res));
    // try to illegally access memory chunks bigger than was allocated
    let res = process.access_addr(0x60000000, 501);
    std::assert!(!check_optional(res));


}

// tests should always use std:: stuff. Int tests maybe not so much
#[cfg(feature = "arctest")]
fn check_optional<T: std::fmt::Display>(optional: Option<T>) -> bool {
    match optional {
        Some(p) => true,
        None => false
    }
}
