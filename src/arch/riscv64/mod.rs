core::arch::global_asm!(include_str!("xv6-trampoline.S"));
core::arch::global_asm!(include_str!("xv6-kernelvec.S"));

use riscv::register::{self, *};

/* -------------
    UART
------------- */

pub const UART0: u64 = 0x1000_0000;
pub const REG_OFFSET: u64 = UART0;

static GREETING: &[u8] = b"Hello World!\n";

macro_rules! write_reg {
    ($reg:expr, $val:expr) => {
        let r = ($reg + REG_OFFSET) as *mut u8;
        unsafe { r.write_volatile($val) }
    };
}

// when debugging, can use uart0 or framebuffer
#[macro_export]
macro_rules! write_uart {
    ($exact:expr) => {
        let p = 0x10000000 as *mut u8;
        let _bytes = $exact;
        for byte in _bytes {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write(p, *byte),
                    _ => core::ptr::write(p, 0xfe),
                }
            }
        }
    };
}

pub fn init_uart() {
    write_reg!(1, 0x00);
    write_reg!(3, 1 << 7);
    write_reg!(0, 0x03);
    write_reg!(1, 0x00);
    write_reg!(3, 3 << 0);
    write_reg!(2, (1 << 0) | (3 << 1));
    write_reg!(1, (1 << 1) | (1 << 0));
    // initlock(&uart_tx_lock, "uart");
}

pub fn test_uart() {
    write_uart!(b"Hello World!\n");
    write_uart!(b"Hello World 2!\n");
}

/* -------------
    XV6 TIMER
------------- */

extern "C" {
    fn timervec();
}

pub fn timerinit() {
    // each CPU has a separate source of timer interrupts.
    let id = mhartid::read();

    // ask the CLINT for a timer interrupt.
    // cycles; about 1/10th second in qemu.
    let interval = 1000000;
    unsafe {
        *(clint_mtimecmp(id as u64) as *mut u64) = *(CLINT_MTIME as *const u64) + interval;
    }

    // 8 cpu system
    let mut timer_scratch = [[0 as u64; 5]; 8];

    // prepare information in scratch[] for timervec.
    let scratch = &mut timer_scratch[id][0];
    unsafe {
        core::ptr::write_volatile((scratch as *mut u64).offset(3), clint_mtimecmp(id as u64));
        core::ptr::write_volatile((scratch as *mut u64).offset(4), interval);

        mscratch::write(scratch as *const _ as usize);
        mtvec::write(timervec as usize, utvec::TrapMode::Direct);
    }

    // enable machine-mode interrupts
}

/* -------------
    XV6 RISCV
------------- */

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

pub const SIE_SEIE: usize = 1 << 9;
pub const SIE_STIE: usize = 1 << 5;
pub const SIE_SSIE: usize = 1 << 1;

pub const KERNBASE: u64 = 0x80000000;
pub const PHYSTOP: u64 = KERNBASE + 128 * 1024 * 1024;

pub const TRAMPOLINE: u64 = MAXVA - PGSIZE;

#[no_mangle]
#[link_section = ".rodata"]
#[used(linker)]
pub static TRAPFRAME: u64 = TRAMPOLINE - PGSIZE;

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

pub fn kstack(pages: u64) -> u64 {
    TRAMPOLINE - (pages + 1) * 2 * PGSIZE
}

/* -------------
    XV6 DEFINED
------------- */

pub const MSTATUS_MPP_MASK: u64 = 3 << 11;
pub const MSTATUS_MPP_M: u64 = 3 << 11;
pub const MSTATUS_MPP_S: u64 = 1 << 11;
pub const MSTATUS_MPP_U: u64 = 0 << 11;
pub const MSTATUS_MIE: u64 = 1 << 3;

pub const PTE_V: u64 = 1 << 0;
pub const PTE_R: u64 = 1 << 1;
pub const PTE_W: u64 = 1 << 2;
pub const PTE_X: u64 = 1 << 3;
pub const PTE_U: u64 = 1 << 4;

pub const PGSIZE: u64 = 4096;
pub const PGSHIFT: u64 = 12;
pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 - 1);
pub const PXMASK: u64 = 0x1FF;

pub fn page_round_up(size: u64) -> u64 {
    ((size) + PGSIZE - 1) & !(PGSIZE - 1)
}
pub fn page_round_down(a: u64) -> u64 {
    (a) & !(PGSIZE - 1)
}
pub fn px_shift(level: u64) -> u64 {
    PGSHIFT + (9 * (level))
}
pub fn px(level: u64, va: u64) -> u64 {
    ((va) >> px_shift(level)) & PXMASK
}
pub fn physical_addr_to_pte_shift(pa: u64) -> u64 {
    ((pa) >> 12) << 10
}
pub fn pte_to_physical_addr(pte: u64) -> u64 {
    ((pte) >> 10) << 12
}
pub fn pte_flags(pte: u64) -> u64 {
    (pte) & 0x3FF
}
