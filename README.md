# Neutron
A rust based kernel built on first principles. Principles called the SPARX principles by yours truly.

## SPARX Principles?
Just a way to say "Shoddy, Pathetic, Abomination, Repetitive, eXceptionally horrendous" to express how software is written.

## Layout
- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic
- sparx/ -> for 1% of the logic including asm, link scripts, external deps (non-rust), build configs for arcboot, useful scripts
- tests/ -> cargo integration tests
- arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now

# Testing
A core idea is TDD. When in doubt, test.

- To run cargo tests, `cargo t` which builds the library for the host target and runs the `#[test]` functions.
- To run vm tests, `arcboot test` builds the a complete image with `feature = arctest` and a custom test harness. It basically runs `rustc test --no-run` for either the spectro/pi target. Then it runs `arcboot run` which boots the vm and loads the image, running the kernel with an arctest config instead of the usual config.

## Cargo Tests
- Mostly for the specific functions (local #[test] in each file) and functions together (things in tests/)
- NOTE: `tests/` is for cargo only and `#[test]` is also for cargo only. Arctest only relies on `feature = arctest` and files in `arctest/`
## Arctest
- Great for validation, blackbox and acceptance testing. Basically any high level stuff that you cant do directly with `cargo t`
- `arcboot test` -> not yet working but will be great for system testing and blackbox testing

# Dependencies
Rust (rustup recommended)
 - rust-src
 - target aarch64 and riscv64 (unknown-none)
 - arcboot
 - spectrovm
Toolchains (add to path or specify sysroot when using `cargo build`)
 - aarch64-gcc
 - riscv64-gcc

## Minimal Config
Since this is a multi target kind of thing in rust, we get a whole bunch of issues if we try to do it the standard way. Recommended to disable any language servers since they can spasm really hard. Maybe theres a way to configure it nicely but Idk I dont really wanna to configure VSCode too much.
- This means things like `.cargo/config.toml` should be very minimal. Mostly for cool things like aliases and stuff. Dont specify any main configs. You can do `[dependencies.X]` for X if you want but I rather leave it mostly vanila and rely on `arcboot` for more complex config and functionality
- Mostly using rust, the language itself and the cargo package management and test suite. I dont really care about the other stuff, at least for now.
