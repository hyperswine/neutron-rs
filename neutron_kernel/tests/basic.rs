use neutron_kernel::process::Process;

extern crate std;

#[test]
fn test_sanity() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn tests_basics() {
    // Process::new(0);
}

#[test]
fn test_elf_load() {
    // load an elf file from tests/elf into the function
    // see whether the function correctly identifies each section
    // and would load it to the right addrs
}
