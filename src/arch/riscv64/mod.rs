pub mod xv6_mem;
// Apparently this doesnt link properly?
// use riscv_rt::entry;

pub const UART0: u64 = 0x1000_0000;
pub const REG_OFFSET: u64 = UART0;

static GREETING: &[u8] = b"Hello World!\n";

// REGISTERS
const IER_REG: u64 = 1;
// queue FIFO for UART streams
const FCR_REG: u64 = 2;
const LCR_REG: u64 = 3;

macro_rules! write_reg {
    ($reg:expr, $val:expr) => {
        let r = ($reg + REG_OFFSET) as *mut u8;
        unsafe { r.write_volatile($val) }
    };
}

// when debugging, can use uart0 or framebuffer
// no color coding though
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
    // disable interrupts
    write_reg!(1, 0x00);

    // BAUD

    // set baud rate (rate of signal change/clock) with line control reg
    write_reg!(3, 1 << 7);
    // least sig bit 0b011
    write_reg!(0, 0x03);
    // most sig bit 0b000
    write_reg!(1, 0x00);

    // OTHER

    // word len. = 8bits
    write_reg!(3, 3 << 0);

    // reset and enable FIFOs (FCR = 2)
    write_reg!(2, (1 << 0) | (3 << 1));

    // enable transmit and receive interrupts
    write_reg!(1, (1 << 1) | (1 << 0));

    // initlock(&uart_tx_lock, "uart");
}

fn set_sp(sp: u64) {
    unsafe { core::arch::asm!("") }
}

fn set_gp(gp: u64) {
    unsafe { core::arch::asm!("") }
}

core::arch::global_asm!(include_str!("xv6-entry.S"));

pub fn begin_riscv() -> ! {
    // init_uart();

    write_uart!(b"Hello World!\n");
    write_uart!(b"Hello World!\n");

    loop {}
}

use riscv::register::{self, mhartid};
use self::xv6_mem::{CLINT_MTIMECMP, CLINT_MTIME, clint_mtimecmp};

// arrange to receive timer interrupts.
// they will arrive in machine mode at
// at timervec in kernelvec.S,
// which turns them into software interrupts for
// devintr() in trap.c.
fn timerinit() {
    // each CPU has a separate source of timer interrupts.
    let id = mhartid::read();

    // ask the CLINT for a timer interrupt.
    let interval = 1000000; // cycles; about 1/10th second in qemu.
    unsafe {
        *(clint_mtimecmp(id.try_into().unwrap()) as *mut u64) = *(CLINT_MTIME as *const u64) + interval;
    }

    // prepare information in scratch[] for timervec.
    // scratch[0..2] : space for timervec to save registers.
    // scratch[3] : address of CLINT MTIMECMP register.
    // scratch[4] : desired interval (in cycles) between timer interrupts.
    let scratch = &mut TIMER_SCRATCH[id][0];
    scratch[3] = CLINT_MTIMECMP(id);
    scratch[4] = interval;
    w_mscratch(scratch as *const _ as u64);

    // set the machine-mode trap handler.
    w_mtvec(timervec as u64);

    // enable machine-mode interrupts
}

pub struct SingleAddressPT {
    n_frames: u64,
    n_free_frames: u64,
    frames: Vec<FrameAddress>,
}

const MAX_FRAMES_64b: usize = 32768;
// kernel only needs 4 frames at boot
const FRAMES_USED_KERN: usize = 4;

impl SingleAddressPT {
    fn new() -> SingleAddressPT {
        SingleAddressPT {
            n_frames: MAX_FRAMES_64b,
            n_free_frames: MAX_FRAMES_64b - FRAMES_USED_KERN,
            frames: FrameAddress::all(),
        }
    }
}

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
