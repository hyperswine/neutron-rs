// Pi 4 Processor Constants

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

// For Pi 4, boot core is always 0
