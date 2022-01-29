// depending on the device being used, link the drivers
// idk how that is done, rn we can use riscv64 and aarch64 for Spike/Spectro and Pi4B
// and just dont support other devices, maybe theres a way to query the hardware id/specs and see if there is a suitable driver, then load it at runtime or something
// a dynamic object Driver being linked and used in Vec<Driver>

// ARCH DEPENDENT

// idk if really needs arch dependent stuff, but at least for spectro hardware im thinking about coupling it somewhat to the kernel

#[cfg(target_arch = "riscv64")]
pub mod spectro;

#[cfg(target_arch = "aarch64")]
pub mod pi4b;

// NON ARCH DEPENDENT

// most should be non arch dependent, e.g. PCIE SSD, HDMI Generic FHD Monitor, Generic Mouse, Generic KB, Generic Headphones, Generic Mic
pub mod generic;

// CODE for dynamic linking in the modules themselves
