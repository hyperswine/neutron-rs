// Translation table.

use crate::{memory::{Address, Physical, Virtual}, types::paging::{MemoryRegion, AttributeFields, PageAddress}};

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
