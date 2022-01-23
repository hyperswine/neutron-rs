// #![feature(custom_test_frameworks)]
#![cfg_attr(not(test), no_std)]

use neutronkern::kernel::KernelManager;

// namespace for integration manager tests
#[cfg(test)]
mod managertests {
    #[test]
    fn test_sanity() {
        let kern_manager = KernelManager;
    }
}
