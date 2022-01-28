// MUST include in lib.rs
// test unit functionality using std crate
extern crate std;

// UNIT TESTS
#[test]
fn trivial_assertion() {
    println!("checking compiler sanity");
    assert_eq!(1, 1);
    println!("nice, basics out of the way!");
}

#[test]
fn test_process() {
    let process = process::Process;
    println!("created a process!");
}

// FILESYSTEM

// mod filesystem;

// TYPES

// mod types;

#[test]
fn test_bytes() {
    let byte = crate::types::Bytes::from_int(50);
    println!("{}", byte.content[0]);
}
