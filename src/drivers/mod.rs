// ARCH DEPENDENT

// idk if really needs arch dependent stuff, but at least for spectro hardware im thinking about coupling it somewhat to the kernel
// for a more optimal xp

// #[cfg(target_arch = "riscv64")]
pub mod spectro;

// #[cfg(target_arch = "aarch64")]
pub mod pi4b;

// NON ARCH DEPENDENT

// most should be non arch dependent, e.g., Generic Mouse, Generic KB, Generic Headphones, Generic Mic
pub mod generic;

// CODE for dynamic linking in the modules themselves

// "Driverless" Components
// PCIE SSD (shouldnt require driver), HDMI Generic FHD Monitor (usually no driver)
// Usually, just send mostly raw data to them

// TODO: QEMU SSD INTERACTION
// detect qemu ssd
// load driver for it
// check if GPT or MBR, then partitions and filesystems
