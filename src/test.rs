// MUST include in lib.rs
// test unit functionality using std crate

// Apaprently some problems with testing on panic = abort
// so have to define your own test framework and hook onto kernel main function
// to run your tests. So when you run cargo test, the cfg(test) gets compiled and we boot the kernel like usual but also run the tests. Then we can make an exit function to exit out of main and close qemu, somehow.
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::test_runner)]
// #![reexport_test_harness_main = "test_main"]

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
