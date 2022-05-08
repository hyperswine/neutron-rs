// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2022 Andre Richter <andre.o.richter@gmail.com>

//! Asynchronous exception handling.

#[cfg(target_arch = "aarch64")]
use crate::kernel::arch::aarch64::exception;

use core::{fmt, marker::PhantomData};

// ------------------------------
// Architectural Public Reexports
// ------------------------------
pub use exception::asynchronous::{
    is_local_irq_masked, local_irq_mask, local_irq_mask_save, local_irq_restore, local_irq_unmask,
};

// ------------------------------
// Definitions
// ------------------------------


#[derive(Copy, Clone)]
pub struct IRQDescriptor {
    pub name: &'static str,
    pub handler: &'static (dyn interface::IRQHandler + Sync),
}


#[derive(Clone, Copy)]
pub struct IRQContext<'irq_context> {
    _0: PhantomData<&'irq_context ()>,
}


pub mod interface {

    pub trait IRQHandler {
        fn handle(&self) -> Result<(), &'static str>;
    }

    pub trait IRQManager {
        type IRQNumberType;

        fn register_handler(
            &self,
            irq_number: Self::IRQNumberType,
            descriptor: super::IRQDescriptor,
        ) -> Result<(), &'static str>;

        
        fn enable(&self, irq_number: Self::IRQNumberType);

        
        fn handle_pending_irqs<'irq_context>(
            &'irq_context self,
            ic: &super::IRQContext<'irq_context>,
        );

        
        fn print_handler(&self);
    }
}


#[derive(Copy, Clone)]
pub struct IRQNumber<const MAX_INCLUSIVE: usize>(usize);

// ---------------
// Implementations
// ---------------

impl<'irq_context> IRQContext<'irq_context> {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        IRQContext { _0: PhantomData }
    }
}

impl<const MAX_INCLUSIVE: usize> IRQNumber<{ MAX_INCLUSIVE }> {
    
    pub const fn new(number: usize) -> Self {
        assert!(number <= MAX_INCLUSIVE);

        Self(number)
    }

    
    pub const fn get(self) -> usize {
        self.0
    }
}

impl<const MAX_INCLUSIVE: usize> fmt::Display for IRQNumber<{ MAX_INCLUSIVE }> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[inline(always)]
pub fn exec_with_irq_masked<T>(f: impl FnOnce() -> T) -> T {
    let ret: T;

    unsafe {
        let saved = local_irq_mask_save();
        ret = f();
        local_irq_restore(saved);
    }

    ret
}
