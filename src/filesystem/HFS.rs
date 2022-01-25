use crate::types::Binary;

// not sure if the one in lib.rs defines it here
// maybe have something at the root of the crate to include
// #[cfg(any(target_arch="riscv64", target_arch="aarch64"))]
// use alloc::{boxed::Box, vec, vec::Vec, rc::Rc, string::String};

// Hierarchical Filesystem
pub struct Filesystem {
    pub files: Vec<File>,
}

type NBits = u64;

pub struct File {
    //in bits, e.g. 10270bits
    pub size: NBits,
    pub name: String,
    created: KTimestamp,
    parent: &Dir,
    last_modified: KTimestamp,
    locked: bool,
    format: FileFormat,
}

struct Dir {
    // common metadata
    created: KTimestamp,
    parent: &File,
    last_modified: KTimestamp,

    // dir specific
    n_items: u64,
    children: Vec<File>,
    parent: Dir,
    // unix operability
    path: String,
}

pub enum FileFormat {
    PDF,
    JPG,
    PNG,
    TXT,
    DOC,
    MD,
    BIN,
    // more stuff
}

struct BinaryFile {
    metadata: File,
    // content: Binary
}

struct AsciiFile {
    pub metadata: File,
    pub content: Vec<u8>,
}

impl Filesystem {
    pub fn new(&self) -> Filesystem {
        let f = File { size: 100 };
        Filesystem { files: Vec::new() }
    }
}
