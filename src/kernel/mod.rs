// ARCH DEPENDENT STUFF

pub mod arch;

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
