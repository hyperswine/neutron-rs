// ARCH DEPENDENT STUFF

pub mod arch;

// INTERFACE TO HANDLE DEPENDENT + INDEPENDENT CODE TOGETHER
// prob cant test directly, at least easily

// NON-ARCH KERNEL STUFF

// extern crate alloc;
// use alloc::vec;

// use crate::filesystem::{Filesystem, File};

// pub struct KernelManager {
//     filesystem: Filesystem
// }

// impl KernelManager {
//     fn k_main(&self) {
//         loop {}
//     }

//     // create a default Kernel Manager with a single empty file (dir) in the HFS
//     pub fn new() -> KernelManager {
//         KernelManager {filesystem: Filesystem{files: vec!(File{size: 0})}}
//     }
// }
