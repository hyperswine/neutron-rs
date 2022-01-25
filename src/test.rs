// MUST include in lib.rs
// test unit functionality using std crate
extern crate std;

// FILESYSTEM

// mod filesystem;

// TYPES

// mod types;

#[test]
fn test_bytes() {
    let byte = crate::types::Bytes::from_int(50);
    println!("{}", byte.content[0]);
}
