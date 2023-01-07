// General Interrupt Controller. Interfaces with the CPU

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

register_bitfields! {
    u32,
    CTLR [
        Enable OFFSET(0) NUMBITS(1) []
    ],
    PMR [
        Priority OFFSET(0) NUMBITS(8) []
    ],
    IAR [
        InterruptID OFFSET(0) NUMBITS(10) []
    ],
    EOIR [
        EOIINTID OFFSET(0) NUMBITS(10) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x000 => CTLR: ReadWrite<u32, CTLR::Register>),
        (0x004 => PMR: ReadWrite<u32, PMR::Register>),
        (0x008 => _reserved1),
        (0x00C => IAR: ReadWrite<u32, IAR::Register>),
        (0x010 => EOIR: ReadWrite<u32, EOIR::Register>),
        (0x014  => @END),
    }
}

type Registers = MMIODerefWrapper<RegisterBlock>;

pub struct GICC {
    registers: InitStateLock<Registers>,
}

impl GICC {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            registers: InitStateLock::new(Registers::new(mmio_start_addr)),
        }
    }

    pub unsafe fn set_mmio(&self, new_mmio_start_addr: usize) {
        self.registers
            .write(|regs| *regs = Registers::new(new_mmio_start_addr));
    }

    pub fn priority_accept_all(&self) {
        self.registers.read(|regs| {
            regs.PMR.write(PMR::Priority.val(255)); // Comment in arch spec.
        });
    }

    pub fn enable(&self) {
        self.registers.read(|regs| {
            regs.CTLR.write(CTLR::Enable::SET);
        });
    }

    pub fn pending_irq_number<'irq_context>(
        &self,
        _ic: &exception::asynchronous::IRQContext<'irq_context>,
    ) -> usize {
        self.registers
            .read(|regs| regs.IAR.read(IAR::InterruptID) as usize)
    }

    pub fn mark_completed<'irq_context>(
        &self,
        irq_number: u32,
        _ic: &exception::asynchronous::IRQContext<'irq_context>,
    ) {
        self.registers.read(|regs| {
            regs.EOIR.write(EOIR::EOIINTID.val(irq_number));
        });
    }
}
