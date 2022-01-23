use neutronkern::types::Binary;

// Semantic Filesystem
pub struct EmberFS {
    files: Vec<File>
}

// Hierarchical Filesystem
pub struct Filesystem {
    files: Vec<File>
}

type NBits = u64;

struct File {
    size: NBits //in bits, e.g. 10270bits
}

struct BinaryFile {
    metadata: File,
    content: Binary
}

struct AsciiFile {
    metadata: File,
    content: Vec<u8>
}

impl Filesytem {
    fn new() -> Filesytem {

    }
}