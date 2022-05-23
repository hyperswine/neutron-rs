# Neutron

A rust based kernel written with the minimalism in mind.

- supports multiboot and stivale2. Should be able to use Limine, GRUB, U-boot, oreboot to boot kernel ELF img in S-Mode/Ring-1
- neutron filesystem used by default
- rei shell is the default shell for interacting with the kernel on neutron arc. qiish is also a great shell for quantii-neutron
- more to come

Neutron is a stablising force. When too many apps are active all competing for the same resources (bonding to a proton), you have an unstable mess that eventually distingerates.

## Boot

Neutron can be built with additive feature support for BIOS, UEFI (stub), Stivale2, Arcboot. YOu just have to enable each with `--features <feature>` on `cargo b`.

## Arcutils

Arcutils has high integration. Just do `arc --ghidra` to analyze it on ghidra.

## Layout

- build/ -> for any temporary build files
- src/ -> for 99% of the kernel logic. src/* first level dirs contain the ABI/API layer for userspace programs like `ls`
- tests/ -> cargo integration tests
- tests/arctests/ -> arctest acceptance tests. #[feature = arctest] are technically system tests even though they are localised, at least for now

## src/*

stuff in kernel/arch/etc. shouldnt depend on stuff in src/.*
stuff in drivers/.* shouldnt depend on stuff in src/.*

maybe except for types/*. Gotta be strict about that

## Arcboot API

Neutron uses the arcboot api by default. When building for aarch64/riscv64-neutron-elf. To build for another boot protocol like bare EFI, multiboot or bare BIOS, enable the features for them. `--features bare_efi`, `--features bare_bios`, `--features multiboot`. Note these features are all mutually exclusive and attempting to use multiple of them at the same time will not compile.

```rust
// in arcboot
// define directive arcboot_entry
extern "C" fn _start() -> ! {
    go_into_s_mode();

    @hook {

    }

    loop {}
}

// params: ArcServices
pub struct ArcServices {
    paging: PageTableTTBR1,
    devices: &'static [ArcDevice],
}

// device (and some drivers like SPI/UART/I2C? And NeFS/FAT32)
pub struct ArcDevice {
    acpi_entry: const* usize,
    device_type: DeviceType
}

// export interfaces
pub enum DeviceType{}

// REMEMBER TO ADD ARCBOOT AS A KEY DEPENDENCY! It will try cargo build I think so you need to either clone the source locally or specify the build procedure to link with vscode rust-analyzer

// in neutron
#[arcboot_entry]
fn main(system_memory: ArcServices) {
    // do what an arcboot app does in S-Mode
}

// drivers
// if detect compatible devices in ArcServices.devices
// load the driver, e.g. BCM, UART, SPIO, NeFS, etc
```
