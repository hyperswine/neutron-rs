pub mod interrupt;
pub mod memory;
pub mod power;
// Apparently this doesnt link properly?
// use riscv_rt::entry;

pub const UART0: u64 = 0x10000000;
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
        let _bytes = $exact.bytes();
        for byte in _bytes {
            unsafe {
                match byte {
                    0x20..=0x7e | b'\n' => core::ptr::write(p, byte),
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

#[cfg(not(test))]
#[no_mangle]
extern "C" fn _start() -> ! {
    init_uart();

    write_uart!(b"Hello World!\n");
    write_uart!(b"Hello World!\n");

    loop {}
}
