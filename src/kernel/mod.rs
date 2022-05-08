// INTERFACE TO HANDLE DEPENDENT + INDEPENDENT CODE TOGETHER

// --------------------
// ARCH DEPENDENT STUFF
// --------------------

pub mod acpi;
pub mod arch;
#[cfg(feature = "posix")]
pub mod posix;
// pub mod console;

// --------------------
// NON-ARCH DEPENDENT STUFF
// --------------------

// Kernel Privilege Level, kind of like CPU but system wide/non arch dependent
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Unknown,
}

use crate::filesystem::VFS;

pub struct KernelManager {
    vfs: VFS::RootFS,
}

impl KernelManager {
    pub fn kernel_manager_entry(&self) -> ! {
        loop {}
    }

    pub fn init(&self) -> ! {
        // CHECK VFS IS IN THE RIGHT FORMAT
        // AND ALL FILES THAT NEED TO BE THERE ARE THERE

        // Now hand off to init to open pseudo terms, start services, etc.
        // Pass off to /sys/init. Make sure it exists. If it doesnt the whole thing should loop and show the error with a 10 second shutdown

        // VFS load_process() into memory
        // let pid = load_process("/sys/init")
        // this_thread.process_run(pid)
        //  /sys/init should be able to spawn more threads using std library/neutron syscalls

        loop {}
    }

    // create a default Kernel Manager with a single empty file (dir) in the HFS
    pub fn new() -> KernelManager {
        KernelManager {
            vfs: VFS::RootFS {},
        }
    }
}

pub fn final_setup() -> ! {
    let kernel_manager = KernelManager::new();
    kernel_manager.init();
}

// --------------------
// TESTS
// --------------------

#[test_case]
fn test_kern_basics() {
    let _kern = KernelManager::new();
}
