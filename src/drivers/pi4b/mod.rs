// Top-level BSP file for the Raspberry Pi 4B

// * NOTE: this is the BSP for Pi 4B Only
// for kernel stuff, use memory/, kernel/arch/aarch64/memory and etc.
// actually Idk if src/memory is really needed. Can just use kernel/arch/...

pub mod console;
pub mod cpu;
pub mod driver;
pub mod exception;
pub mod memory;

use super::arm::bcm::PL011Uart;

static GPIO: GPIO = unsafe { GPIO::new(MMIODescriptor::new(GPIO_START, GPIO_SIZE)) };

static PL011_UART: PL011Uart = unsafe {
    PL011Uart::new(
        MMIODescriptor::new(PL011_UART_START, PL011_UART_SIZE),
        PL011_UART,
    )
};

static INTERRUPT_CONTROLLER: GICv2 = unsafe {
    GICv2::new(
        MMIODescriptor::new(GICD_START, GICD_SIZE),
        MMIODescriptor::new(GICC_START, GICC_SIZE),
    )
};

pub fn board_name() -> &'static str {
    "Raspberry Pi 4"
}
