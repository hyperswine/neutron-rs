// enable by default for testing
// A lot of graphics functionality is in userspace arc/graphics
// #[cfg(feature = "graphics")]
pub mod graphics;

// #[cfg(feature = "driver_ext")]
pub mod driver_extensions;

// Arc VMs require hypervisor and container support
// #[cfg(feature = "hypervisor")]
pub mod hypervisor;

// #[cfg(feature = "container")]
pub mod container;

// --------------------
// ARCH DEPENDENT STUFF
// --------------------

#[cfg(feature = "posix")]
pub mod posix;
