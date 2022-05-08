// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>

// BSP Memory Management.
pub mod mmu;

use crate::memory::{mmu::PageAddress, Address, Physical, Virtual};
use core::cell::UnsafeCell;

// Private Definitions

// Symbols from the linker script.
extern "Rust" {
    static __code_start: UnsafeCell<()>;
    static __code_end_exclusive: UnsafeCell<()>;

    static __data_start: UnsafeCell<()>;
    static __data_end_exclusive: UnsafeCell<()>;

    static __mmio_remap_start: UnsafeCell<()>;
    static __mmio_remap_end_exclusive: UnsafeCell<()>;

    static __boot_core_stack_start: UnsafeCell<()>;
    static __boot_core_stack_end_exclusive: UnsafeCell<()>;
}

// Public Definitions


pub(super) mod map {
    use super::*;

    
    #[cfg(feature = "bsp_rpi3")]
    pub mod mmio {
        use super::*;

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

    
    #[cfg(feature = "bsp_rpi4")]
    pub mod mmio {
        use super::*;

        pub const GPIO_START: Address<Physical> = Address::new(0xFE20_0000);
        pub const GPIO_SIZE: usize = 0xA0;

        pub const PL011_UART_START: Address<Physical> = Address::new(0xFE20_1000);
        pub const PL011_UART_SIZE: usize = 0x48;

        pub const GICD_START: Address<Physical> = Address::new(0xFF84_1000);
        pub const GICD_SIZE: usize = 0x824;

        pub const GICC_START: Address<Physical> = Address::new(0xFF84_2000);
        pub const GICC_SIZE: usize = 0x14;

        pub const END: Address<Physical> = Address::new(0xFF85_0000);
    }

    pub const END: Address<Physical> = mmio::END;
}

// Private Code


#[inline(always)]
fn virt_code_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __code_start.get() as usize })
}


#[inline(always)]
fn code_size() -> usize {
    unsafe { (__code_end_exclusive.get() as usize) - (__code_start.get() as usize) }
}


#[inline(always)]
fn virt_data_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __data_start.get() as usize })
}


#[inline(always)]
fn data_size() -> usize {
    unsafe { (__data_end_exclusive.get() as usize) - (__data_start.get() as usize) }
}


#[inline(always)]
fn virt_mmio_remap_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __mmio_remap_start.get() as usize })
}


#[inline(always)]
fn mmio_remap_size() -> usize {
    unsafe { (__mmio_remap_end_exclusive.get() as usize) - (__mmio_remap_start.get() as usize) }
}


#[inline(always)]
fn virt_boot_core_stack_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __boot_core_stack_start.get() as usize })
}


#[inline(always)]
fn boot_core_stack_size() -> usize {
    unsafe {
        (__boot_core_stack_end_exclusive.get() as usize) - (__boot_core_stack_start.get() as usize)
    }
}

// Public Code


#[inline(always)]
pub fn phys_addr_space_end_exclusive_addr() -> PageAddress<Physical> {
    PageAddress::from(map::END)
}
