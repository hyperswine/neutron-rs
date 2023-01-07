pub mod irq_map {
    pub const PL011_UART: IRQNumber = IRQNumber::Peripheral(PeripheralIRQ::new(57));
}

pub mod mmio {
    pub const PERIPHERAL_IC_START: Address<Physical> = Address::new(0x3F00_B200);
    pub const PERIPHERAL_IC_SIZE: usize = 0x24;

    pub const GPIO_START: Address<Physical> = Address::new(0x3F20_0000);
    pub const GPIO_SIZE: usize = 0xA0;

    pub const PL011_UART_START: Address<Physical> = Address::new(0x3F20_1000);
    pub const PL011_UART_SIZE: usize = 0x48;

    pub const LOCAL_IC_START: Address<Physical> = Address::new(0x4000_0000);
    pub const LOCAL_IC_SIZE: usize = 0x100;

    pub const END: Address<Physical> = Address::new(0x4001_0000);
}

static INTERRUPT_CONTROLLER: device_driver::InterruptController = unsafe {
    device_driver::InterruptController::new(
        MMIODescriptor::new(mmio::LOCAL_IC_START, mmio::LOCAL_IC_SIZE),
        MMIODescriptor::new(mmio::PERIPHERAL_IC_START, mmio::PERIPHERAL_IC_SIZE),
    )
};
