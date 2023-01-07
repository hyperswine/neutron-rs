use neutron_kernel::process::{elf::load_elf_userspace, Process};
use std::fs;

#[test]
fn test_sanity() {
    assert_eq!(1 + 1, 2);
}

#[test]
fn test_elf_load_section_and_identify() {
    log::warn!("This is an example message.");

    /*
    I got `cm.so` from user `S01den` [here](https://crackmes.one/crackme/62d08a7a33c5d44a934e97bb).
    I got `keygen_me` from [Deskarponne](https://crackmes.one/user/Deskarponne).
    */

    let f = fs::read("tests/elf/keygen_me").unwrap();

    load_elf_userspace(&f);
}
