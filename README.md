# Neutron

A rust based kernel written with the minimalism in mind.

## TODO

- make cargo test work on riscv and arm with a qemu runner
- maybe requires xtask to do it properly. Otherwise I guess `arcutils test --elf-img build/neutron_kernel` could work

## Layout

- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic
- tests/ -> cargo integration tests
- tests/arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now
