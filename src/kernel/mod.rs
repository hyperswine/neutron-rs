// INTERFACE TO HANDLE DEPENDENT + INDEPENDENT CODE TOGETHER

// --------------------
// ARCH DEPENDENT STUFF
// --------------------

pub mod arch;
pub mod acpi;
#[cfg(feature = "posix")]
pub mod posix;

// --------------------
// NON-ARCH DEPENDENT STUFF
// --------------------

use alloc::vec;

use crate::filesystem::hfs_v1::{File, Filesystem};

pub struct KernelManager {
    filesystem: Filesystem,
}

impl KernelManager {
    fn k_main(&self) {
        loop {}
    }

    // create a default Kernel Manager with a single empty file (dir) in the HFS
    pub fn new() -> KernelManager {
        KernelManager {
            filesystem: Filesystem::new(),
        }
    }
}

// --------------------
// TESTS
// --------------------

#[test_case]
fn test_kern_basics() {
    let _kern = KernelManager::new();
}
