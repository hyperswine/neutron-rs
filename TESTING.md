# Testing

Basically, build with `cargo t*` and then run the test config with:

```bash
qemu-system-riscv64 -M virt -kernel build/neutron_kernel -nographic -serial mon:stdio
```

## TDD

A core idea is TDD, write tests for expected behavior then implement functionality until the tests passes.

- To run cargo tests, `cargo t` which builds the library for the host target and runs the `#[test_case]` functions.
- To run vm tests, `arcboot test` builds the a complete image with `feature = arctest` and a custom test harness. It basically runs `rustc test --no-run` for either the spectro/pi target. Then it runs `arcboot run` which boots the vm and loads the image, running the kernel with an arctest config instead of the usual config.

## Cargo Tests

Like building, we test for a specific arch. So `cargo t[arm|rv|x86]`. For each of these platforms, we build a test-configured ELF binary that can be run on QEMU.

Cargo test uses the `qemu` runner config to run the image directly and output `debug!` messages to the host console.

- Mostly for the specific functions (local #[test_case] in each file) and functions together (things in tests/)
- It should start up QEMU, run each test including integration tests, then quit
- Each test should show a pass/fail on the host console and any warnings or panic-halt messages

NOTE: `tests/` is for cargo only and `#[test_case]` is also for cargo only. Arctest only relies on `feature = arctest` and files in `tests/arctest/`

## Arctest

- Great for validation, blackbox and acceptance testing. Basically any high level stuff that you cant do directly with `cargo t`
- `arcboot test` -> not yet working but will be great for system testing and blackbox testing
