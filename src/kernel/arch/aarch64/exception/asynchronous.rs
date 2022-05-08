use core::arch::asm;
use cortex_a::registers::*;
use tock_registers::interfaces::{Readable, Writeable};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

mod daif_bits {
    pub const IRQ: u8 = 0b0010;
}

trait DaifField {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register>;
}

struct Debug;
struct SError;
struct IRQ;
struct FIQ;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

impl DaifField for Debug {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::D
    }
}

impl DaifField for SError {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::A
    }
}

impl DaifField for IRQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::I
    }
}

impl DaifField for FIQ {
    fn daif_field() -> tock_registers::fields::Field<u64, DAIF::Register> {
        DAIF::F
    }
}

fn is_masked<T>() -> bool
where
    T: DaifField,
{
    DAIF.is_set(T::daif_field())
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------


pub fn is_local_irq_masked() -> bool {
    !is_masked::<IRQ>()
}












#[inline(always)]
pub unsafe fn local_irq_unmask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}






#[inline(always)]
pub unsafe fn local_irq_mask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}






#[inline(always)]
pub unsafe fn local_irq_mask_save() -> u64 {
    let saved = DAIF.get();
    local_irq_mask();

    saved
}




#[inline(always)]
pub unsafe fn local_irq_restore(saved: u64) {
    DAIF.set(saved);
}

// Print the AArch64 exceptions status.
// #[rustfmt::skip]
// pub fn print_state() {
//     use crate::info;

//     let to_mask_str = |x| -> _ {
//         if x { "Masked" } else { "Unmasked" }
//     };

//     info!("      Debug:  {}", to_mask_str(is_masked::<Debug>()));
//     info!("      SError: {}", to_mask_str(is_masked::<SError>()));
//     info!("      IRQ:    {}", to_mask_str(is_masked::<IRQ>()));
//     info!("      FIQ:    {}", to_mask_str(is_masked::<FIQ>()));
// }
