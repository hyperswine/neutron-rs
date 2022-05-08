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

static GPIO: device_driver::GPIO =
    unsafe { device_driver::GPIO::new(MMIODescriptor::new(mmio::GPIO_START, mmio::GPIO_SIZE)) };

static PL011_UART: PL011Uart = unsafe {
    device_driver::PL011Uart::new(
        MMIODescriptor::new(mmio::PL011_UART_START, mmio::PL011_UART_SIZE),
        exception::asynchronous::irq_map::PL011_UART,
    )
};



#[cfg(feature = "bsp_rpi4")]
static INTERRUPT_CONTROLLER: device_driver::GICv2 = unsafe {
    device_driver::GICv2::new(
        MMIODescriptor::new(mmio::GICD_START, mmio::GICD_SIZE),
        MMIODescriptor::new(mmio::GICC_START, mmio::GICC_SIZE),
    )
};

pub fn board_name() -> &'static str {
    "Raspberry Pi 4"
}
