pub const UART0: u64 = 0x10000000;
pub const UART0_IRQ: u64 = 10;

pub const VIRTIO0: u64 = 0x10001000;
pub const VIRTIO0_IRQ: u64 = 1;

pub const CLINT: u64 = 0x2000000;
pub const CLINT_MTIMECMP: u64 = CLINT + 0x4000;
pub const CLINT_MTIME: u64 = CLINT + 0xBFF8;

pub const PLIC: u64 = 0x0c000000;
pub const PLIC_PRIORITY: u64 = PLIC + 0x0;
pub const PLIC_PENDING: u64 = PLIC + 0x1000;
pub const PLIC_MPRIORITY: u64 = PLIC + 0x200000;
pub const PLIC_SPRIORITY: u64 = PLIC + 0x201000;
pub const PLIC_MCLAIM: u64 = PLIC + 0x200004;
pub const PLIC_SCLAIM: u64 = PLIC + 0x201004;

pub const KERNBASE: u64 = 0x80000000;
pub const PHYSTOP: u64 = KERNBASE + 128 * 1024 * 1024;

pub const TRAMPOLINE: u64 = MAXVA - PGSIZE;

pub const TRAPFRAME: u64 = TRAMPOLINE - PGSIZE;

pub fn clint_mtimecmp(hartid: u64) -> u64 {
    CLINT_MTIMECMP + 8 * hartid
}

pub fn plic_menable(hart: u64) -> u64 {
    PLIC + 0x2000 + hart * 0x100
}

pub fn plic_senable(hart: u64) -> u64 {
    PLIC + 0x2080 + hart * 0x100
}

pub fn plic_mpriority(hart: u64) -> u64 {
    PLIC + 0x200000 + hart * 0x2000
}

pub fn plic_spriority(hart: u64) -> u64 {
    PLIC + 0x201000 + hart * 0x2000
}

pub fn plic_mclaim(hart: u64) -> u64 {
    PLIC + 0x200004 + hart * 0x2000
}

pub fn plic_sclaim(hart: u64) -> u64 {
    PLIC + 0x201004 + hart * 0x2000
}

pub const PGSIZE: u64 = 4096;
pub const PGSHIFT: u64 = 12;

pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 - 1);

pub fn kstack(pages: u64) -> u64 {
    TRAMPOLINE - (pages + 1) * 2 * PGSIZE
}
