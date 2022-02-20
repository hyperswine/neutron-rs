// Semantic Filesystem
// Built on top of HFS

#[cfg(not(test))]
use alloc::vec::Vec;

use crate::filesystem::HFS::File;

pub struct EmberFS {
    files: Vec<File>
}
