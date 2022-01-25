// use crate::types::Binary;

// !TEMP: dynamically allocated stuff

// Hierarchical Filesystem
pub struct Filesystem {
    pub files: Vec<File>
}

type NBits = u64;

pub struct File {
    pub size: NBits //in bits, e.g. 10270bits
}

struct BinaryFile {
    metadata: File,
    // content: Binary
}

struct AsciiFile {
    pub metadata: File,
    pub content: Vec<u8>
}

impl Filesystem {
    pub fn new(&self) -> Filesystem {
        let f = File{size: 100};
        Filesystem{files: Vec::new()}
    }
}