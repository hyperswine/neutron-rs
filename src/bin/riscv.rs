#![no_std]
#![no_main]

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use neutron_kernel::arch::riscv64::{
    timerinit, MSTATUS_MPP_MASK, MSTATUS_MPP_S, SIE_SEIE, SIE_SSIE, SIE_STIE,
};
use riscv::register::{self, *};

#[no_mangle]
extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "
        la sp, stack
        li a0, 1024*4
        csrr a1, mhartid
        addi a1, a1, 1
        mul a0, a0, a1
        add sp, sp, a0
    "
        );
    }

    // set M Previous Privilege mode to Supervisor, for mret.
    // turn on supervisor flag, turn off 6144 -> sie
    // turn off User Previous Interrupt Enable -> upie
    unsafe {
        // ?Idk if this should be cleared or set
        mstatus::set_upie();
        mstatus::set_sie();
        mepc::write(kernel_main as usize);

        // disable paging for now.
        satp::write(0);

        // delegate all interrupts and exceptions to supervisor mode.
        write_medeleg(0xffff);
        write_mideleg(0xffff);
        // sie::write(sie::read().bits() | SIE_SEIE | SIE_STIE | SIE_SSIE);
        sie::set_sext();
        sie::set_ssoft();
        sie::set_stimer();

        // configure Physical Memory Protection to give supervisor mode
        pmpaddr0::write(0x3fffffffffffff);
        pmpcfg0::write(0xf);
    }

    // ask for clock interrupts.
    timerinit();

    // keep each CPU's hartid in its tp register, for cpuid().
    let id = mhartid::read();
    write_thread_pointer(id as u64);

    // switch to supervisor mode and jump to main().
    unsafe {
        core::arch::asm!("mret");
    }

    loop {}
}

extern "C" fn kernel_main() {}

// RISCV REG

fn read_thread_pointer() -> u64 {
    let mut x: u64;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) x);
    }
    x
}

fn write_thread_pointer(x: u64) {
    unsafe {
        core::arch::asm!("mv tp, {}", in(reg) x);
    }
}

use core::arch::asm;

fn write_medeleg(x: u64) {
    unsafe {
        asm!("csrw medeleg, {}", in(reg) x);
    }
}

fn write_mideleg(x: u64) {
    unsafe {
        asm!("csrw mideleg, {}", in(reg) x);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();
