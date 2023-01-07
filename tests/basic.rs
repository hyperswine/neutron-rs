use neutron_kernel::process::{elf::load_elf_userspace, Process};
use std::fs;

/*
I got `cm.so` from user `S01den` [here](https://crackmes.one/crackme/62d08a7a33c5d44a934e97bb).
I got `keygen_me` from [Deskarponne](https://crackmes.one/user/Deskarponne).
*/

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
    // SimpleLogger::new().env().init().unwrap();
    log::warn!("This is an example message.");

    let f = fs::read("tests/elf/keygen_me").unwrap();

    load_elf_userspace(&f);
}
