// NOTE, not(test) basically means anything other than the host
use alloc::{boxed::Box, rc::Rc, string::String, vec, vec::Vec, string::ToString};
// KERNEL IMPORTS
use crate::types::KTimestamp;

// ------------------------
// Hierarchical Filesystem
// ------------------------

// Basically a generic hierarchical filesystem for testing

pub struct Filesystem {
    pub files: Vec<File>,
}

type NBits = u64;
type NBytes = u64;

pub struct File {
    // in bits, e.g. 1024B
    size: NBytes,
    name: String,
    created: KTimestamp,

    // parent: *const Dir // for faster recognition, though more metadata
    last_modified: KTimestamp,
    locked: bool,
    format: FileFormat,
    path: String,
}

struct Dir {
    // common metadata
    created: KTimestamp,
    last_modified: KTimestamp,

    // dir specific
    n_items: u64,
    children: Vec<File>,
    parent: *const Dir,
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
    // ...more stuff
}

struct BinaryFile {
    metadata: File,
    // content: &[u8]
}

struct AsciiFile {
    pub metadata: File,
    pub content: Vec<u8>,
}

impl Filesystem {
    pub fn new() -> Self {
        // TECHNICALLY A DIR
        let f = File::new("/root");
        Self { files: Vec::new() }
    }
}

impl File {
    pub fn new(filename: &str) -> Self {
        Self {
            size: 1,
            name: filename.to_string(),
            // TODO: check it before Self{} instead of unwrapping
            created: KTimestamp::from_yyyy_mm_dd("").unwrap(),
            last_modified: KTimestamp::from_yyyy_mm_dd("").unwrap(),
            locked: false,
            format: FileFormat::TXT,
            path: "/".to_string()
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// ------------------------
// Unit Tests
// ------------------------

// #[test]
// fn test_file() {
//     let file = File::new("file");
//     let name = file.get_name();
//     assert_eq!(name, "file");
// }
