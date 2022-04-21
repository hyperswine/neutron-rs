# Neutron

A rust based kernel written with the minimalism in mind.

- multiboot compliant. Should be able to use GRUB, U-boot, oreboot to boot kernel ELF img in S-Mode/Ring-1
- neutron filesystem used by default
- rei shell is the default shell for interacting with the kernel on neutron arc
- more to come

## Layout

- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic
- tests/ -> cargo integration tests
- tests/arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now
