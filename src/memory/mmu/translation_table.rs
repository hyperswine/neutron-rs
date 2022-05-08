// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

// Translation table.

// TODO: kernel/arch/aarch64

#[cfg(target_arch = "aarch64")]
mod translation;

// Architectural Public Reexports
#[cfg(target_arch = "aarch64")]
pub use arch_translation_table::FixedSizeTranslationTable;

use super::{AttributeFields, MemoryRegion};
use crate::memory::{Address, Physical, Virtual};

// Public Definitions


pub mod interface {
    use crate::memory::mmu::PageAddress;

    use super::*;

    
    pub trait TranslationTable {
        fn init(&mut self) -> Result<(), &'static str>;

        
        unsafe fn map_at(
            &mut self,
            virt_region: &MemoryRegion<Virtual>,
            phys_region: &MemoryRegion<Physical>,
            attr: &AttributeFields,
        ) -> Result<(), &'static str>;

        
        fn try_virt_page_addr_to_phys_page_addr(
            &self,
            virt_page_addr: PageAddress<Virtual>,
        ) -> Result<PageAddress<Physical>, &'static str>;

        
        fn try_page_attributes(
            &self,
            virt_page_addr: PageAddress<Virtual>,
        ) -> Result<AttributeFields, &'static str>;

        
        fn try_virt_addr_to_phys_addr(
            &self,
            virt_addr: Address<Virtual>,
        ) -> Result<Address<Physical>, &'static str>;
    }
}
