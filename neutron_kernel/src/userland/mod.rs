// INTERFACE FOR CONTROLLING ALL KERNEL MODULES TOGETHER AND LAUNCHING USERSPACE

// Kernel Privilege Level, kind of like CPU but system wide/non arch dependent
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    Machine,
    User,
    Kernel,
    Hypervisor,
    Unknown,
}

use crate::filesystem::vfs;

pub struct KernelTorch {
    vfs: vfs::RootFS,
}

impl KernelTorch {
    pub fn pass_torch(&self) -> ! {
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
    pub fn new() -> KernelTorch {
        KernelTorch {
            vfs: vfs::RootFS {},
        }
    }
}

pub fn final_setup() -> ! {
    let kernel_manager = KernelTorch::new();
    kernel_manager.init();
}

// --------------------
// TESTS
// --------------------

#[test]
fn test_userland_pass() {
    let _kern = KernelTorch::new();
}
