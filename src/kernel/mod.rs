// ARCH DEPENDENT STUFF

pub mod arch;
pub mod entry;
// pub mod posix; // not finished

// INTERFACE TO HANDLE DEPENDENT + INDEPENDENT CODE TOGETHER

// NON-ARCH KERNEL STUFF

// ! Will be a bit hard to test directly, dont write integration tests for these modules that rely on alloc
// Unless you can specify your own allocator based on paging somehow

#[cfg(not(test))]
use alloc::vec;

use crate::filesystem::HFS::{File, Filesystem};

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

#[cfg(feature = "posix")]
pub mod posix;
