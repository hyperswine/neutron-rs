#[cfg(target_arch = "aarch64")]
use crate::arch::aarch64::exception;

#[cfg(target_arch = "riscv")]
use crate::arch::riscv64gc::exception;

#[cfg(target_arch = "x86_64")]
use crate::arch::x86::exception;

// ABOVE: BAD IDEA, instead, call these functions from arch
// And pass off to common code ASAP

use core::{fmt, marker::PhantomData};

// -----------
// Reexports
// -----------

pub use exception::{
    is_local_irq_masked, local_irq_mask, local_irq_mask_save, local_irq_restore, local_irq_unmask,
};

// ------------
// IRQ
// ------------

#[derive(Copy, Clone)]
pub struct IRQDescriptor {
    pub name: &'static str,
    pub handler: &'static (dyn IRQHandler + Sync),
}

#[derive(Clone, Copy)]
pub struct IRQContext<'irq_context> {
    _0: PhantomData<&'irq_context ()>,
}

pub trait IRQHandler {
    fn handle(&self) -> Result<(), &'static str>;
}

/// Arch specific code should register its own IRQ handler
pub trait IRQManager {
    type IRQNumberType;

    fn register_handler(
        &self,
        irq_number: Self::IRQNumberType,
        descriptor: IRQDescriptor,
    ) -> Result<(), &'static str>;

    fn enable(&self, irq_number: Self::IRQNumberType);

    fn handle_pending_irqs<'irq_context>(&'irq_context self, ic: &IRQContext<'irq_context>);

    fn print_handler(&self);
}

#[derive(Copy, Clone)]
pub struct IRQNumber<const MAX_INCLUSIVE: usize>(usize);

// ---------------
// IRQ Implementations
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
