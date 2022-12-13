use arcboot_api::ArcServices;
use neutron_kernel::{arch::aarch64::entry::arch_init, kernel::common};

// An arcboot app is able to return
// arcboot_entry -> no mangles it. Basically main() but without rust doing weird things
// Cant be bothered writing an [arc_entry] macro

extern "C" fn arc_entry(arcservices: ArcServices) {
    #[cfg(target_arch = "aarch64")]
    arch_init(arcservices);

    // SHOULD BE CALLED BY THE ARCH INIT CODE, or maybe after the arch init code, it returns here
    common();
}
