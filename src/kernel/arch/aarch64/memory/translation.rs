use core::convert;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]
        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],
        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],
        AttrIndx OFFSET(2) NUMBITS(3) [],
        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],
        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[derive(Copy, Clone)]
#[repr(C)]
struct TableDescriptor {
    value: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct PageDescriptor {
    value: u64,
}

trait StartAddr {
    fn virt_start_addr(&self) -> Address<Virtual>;
}

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize, const START_FROM_TOP: bool> {
    lvl3: [[PageDescriptor; 8192]; NUM_TABLES],

    lvl2: [TableDescriptor; NUM_TABLES],

    initialized: bool,
}

impl<T, const N: usize> StartAddr for [T; N] {
    fn virt_start_addr(&self) -> Address<Virtual> {
        Address::new(self as *const _ as usize)
    }
}

impl TableDescriptor {
    pub const fn new_zeroed() -> Self {
        Self { value: 0 }
    }

    pub fn from_next_lvl_table_addr(phys_next_lvl_table_addr: Address<Physical>) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_next_lvl_table_addr.as_usize() >> Granule64KiB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB.val(shifted as u64)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );

        TableDescriptor { value: val.get() }
    }
}

impl convert::From<AttributeFields>
    for tock_registers::fields::FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register>
{
    fn from(attribute_fields: AttributeFields) -> Self {
        // Memory attributes.
        let mut desc = match attribute_fields.mem_attributes {
            MemAttributes::CacheableDRAM => {
                STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(memory::mmu::arch_mmu::mair::NORMAL)
            }
            MemAttributes::Device => {
                STAGE1_PAGE_DESCRIPTOR::SH::OuterShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(memory::mmu::arch_mmu::mair::DEVICE)
            }
        };

        desc += match attribute_fields.acc_perms {
            AccessPermissions::ReadOnly => STAGE1_PAGE_DESCRIPTOR::AP::RO_EL1,
            AccessPermissions::ReadWrite => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1,
        };

        desc += if attribute_fields.execute_never {
            STAGE1_PAGE_DESCRIPTOR::PXN::True
        } else {
            STAGE1_PAGE_DESCRIPTOR::PXN::False
        };

        desc += STAGE1_PAGE_DESCRIPTOR::UXN::True;

        desc
    }
}

impl convert::TryFrom<InMemoryRegister<u64, STAGE1_PAGE_DESCRIPTOR::Register>> for AttributeFields {
    type Error = &'static str;

    fn try_from(
        desc: InMemoryRegister<u64, STAGE1_PAGE_DESCRIPTOR::Register>,
    ) -> Result<AttributeFields, Self::Error> {
        let mem_attributes = match desc.read(STAGE1_PAGE_DESCRIPTOR::AttrIndx) {
            memory::mmu::arch_mmu::mair::NORMAL => MemAttributes::CacheableDRAM,
            memory::mmu::arch_mmu::mair::DEVICE => MemAttributes::Device,
            _ => return Err("Unexpected memory attribute"),
        };

        let acc_perms = match desc.read_as_enum(STAGE1_PAGE_DESCRIPTOR::AP) {
            Some(STAGE1_PAGE_DESCRIPTOR::AP::Value::RO_EL1) => AccessPermissions::ReadOnly,
            Some(STAGE1_PAGE_DESCRIPTOR::AP::Value::RW_EL1) => AccessPermissions::ReadWrite,
            _ => return Err("Unexpected access permission"),
        };

        let execute_never = desc.read(STAGE1_PAGE_DESCRIPTOR::PXN) > 0;

        Ok(AttributeFields {
            mem_attributes,
            acc_perms,
            execute_never,
        })
    }
}

impl PageDescriptor {
    pub const fn new_zeroed() -> Self {
        Self { value: 0 }
    }

    pub fn from_output_page_addr(
        phys_output_page_addr: PageAddress<Physical>,
        attribute_fields: &AttributeFields,
    ) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_output_page_addr.into_inner().as_usize() >> Granule64KiB::SHIFT;
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_64KiB.val(shifted as u64)
                + STAGE1_PAGE_DESCRIPTOR::AF::True
                + STAGE1_PAGE_DESCRIPTOR::TYPE::Page
                + STAGE1_PAGE_DESCRIPTOR::VALID::True
                + (*attribute_fields).into(),
        );

        Self { value: val.get() }
    }

    fn is_valid(&self) -> bool {
        InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(self.value)
            .is_set(STAGE1_PAGE_DESCRIPTOR::VALID)
    }

    fn output_page_addr(&self) -> PageAddress<Physical> {
        let shifted = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(self.value)
            .read(STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_64KiB) as usize;

        PageAddress::from(shifted << Granule64KiB::SHIFT)
    }

    fn try_attributes(&self) -> Result<AttributeFields, &'static str> {
        InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(self.value).try_into()
    }
}

impl<const AS_SIZE: usize> memory::mmu::AssociatedTranslationTable
    for memory::mmu::AddressSpace<AS_SIZE>
where
    [u8; Self::SIZE >> Granule512MiB::SHIFT]: Sized,
{
    type TableStartFromTop =
        FixedSizeTranslationTable<{ Self::SIZE >> Granule512MiB::SHIFT }, true>;

    type TableStartFromBottom =
        FixedSizeTranslationTable<{ Self::SIZE >> Granule512MiB::SHIFT }, false>;
}

impl<const NUM_TABLES: usize, const START_FROM_TOP: bool>
    FixedSizeTranslationTable<NUM_TABLES, START_FROM_TOP>
{
    const START_FROM_TOP_OFFSET: Address<Virtual> =
        Address::new((usize::MAX - (Granule512MiB::SIZE * NUM_TABLES)) + 1);

    const fn _new(for_precompute: bool) -> Self {
        Self {
            lvl3: [[PageDescriptor::new_zeroed(); 8192]; NUM_TABLES],
            lvl2: [TableDescriptor::new_zeroed(); NUM_TABLES],
            initialized: for_precompute,
        }
    }

    pub const fn new_for_precompute() -> Self {
        Self::_new(true)
    }

    #[cfg(test)]
    pub fn new_for_runtime() -> Self {
        Self::_new(false)
    }

    #[inline(always)]
    fn lvl2_lvl3_index_from_page_addr(
        &self,
        virt_page_addr: PageAddress<Virtual>,
    ) -> Result<(usize, usize), &'static str> {
        let mut addr = virt_page_addr.into_inner();

        if START_FROM_TOP {
            addr = addr - Self::START_FROM_TOP_OFFSET;
        }

        let lvl2_index = addr.as_usize() >> Granule512MiB::SHIFT;
        let lvl3_index = (addr.as_usize() & Granule512MiB::MASK) >> Granule64KiB::SHIFT;

        if lvl2_index > (NUM_TABLES - 1) {
            return Err("Virtual page is out of bounds of translation table");
        }

        Ok((lvl2_index, lvl3_index))
    }

    #[inline(always)]
    fn page_descriptor_from_page_addr(
        &self,
        virt_page_addr: PageAddress<Virtual>,
    ) -> Result<&PageDescriptor, &'static str> {
        let (lvl2_index, lvl3_index) = self.lvl2_lvl3_index_from_page_addr(virt_page_addr)?;
        let desc = &self.lvl3[lvl2_index][lvl3_index];

        Ok(desc)
    }

    #[inline(always)]
    fn set_page_descriptor_from_page_addr(
        &mut self,
        virt_page_addr: PageAddress<Virtual>,
        new_desc: &PageDescriptor,
    ) -> Result<(), &'static str> {
        let (lvl2_index, lvl3_index) = self.lvl2_lvl3_index_from_page_addr(virt_page_addr)?;
        let desc = &mut self.lvl3[lvl2_index][lvl3_index];

        if desc.is_valid() {
            return Err("Virtual page is already mapped");
        }

        *desc = *new_desc;
        Ok(())
    }
}

//-------------------
// OS Interface Code
//-------------------

impl<const NUM_TABLES: usize, const START_FROM_TOP: bool>
    memory::mmu::translation_table::interface::TranslationTable
    for FixedSizeTranslationTable<NUM_TABLES, START_FROM_TOP>
{
    fn init(&mut self) -> Result<(), &'static str> {
        if self.initialized {
            return Ok(());
        }

        for (lvl2_nr, lvl2_entry) in self.lvl2.iter_mut().enumerate() {
            let virt_table_addr = self.lvl3[lvl2_nr].virt_start_addr();
            let phys_table_addr = memory::mmu::try_kernel_virt_addr_to_phys_addr(virt_table_addr)?;

            let new_desc = TableDescriptor::from_next_lvl_table_addr(phys_table_addr);
            *lvl2_entry = new_desc;
        }

        self.initialized = true;

        Ok(())
    }

    unsafe fn map_at(
        &mut self,
        virt_region: &MemoryRegion<Virtual>,
        phys_region: &MemoryRegion<Physical>,
        attr: &AttributeFields,
    ) -> Result<(), &'static str> {
        assert!(self.initialized, "Translation tables not initialized");

        if virt_region.size() != phys_region.size() {
            return Err("Tried to map memory regions with unequal sizes");
        }

        if phys_region.end_exclusive_page_addr() > memory::phys_addr_space_end_exclusive_addr()
        {
            return Err("Tried to map outside of physical address space");
        }

        let iter = phys_region.into_iter().zip(virt_region.into_iter());
        for (phys_page_addr, virt_page_addr) in iter {
            let new_desc = PageDescriptor::from_output_page_addr(phys_page_addr, attr);
            let virt_page = virt_page_addr;

            self.set_page_descriptor_from_page_addr(virt_page, &new_desc)?;
        }

        Ok(())
    }

    fn try_virt_page_addr_to_phys_page_addr(
        &self,
        virt_page_addr: PageAddress<Virtual>,
    ) -> Result<PageAddress<Physical>, &'static str> {
        let page_desc = self.page_descriptor_from_page_addr(virt_page_addr)?;

        if !page_desc.is_valid() {
            return Err("Page marked invalid");
        }

        Ok(page_desc.output_page_addr())
    }

    fn try_page_attributes(
        &self,
        virt_page_addr: PageAddress<Virtual>,
    ) -> Result<AttributeFields, &'static str> {
        let page_desc = self.page_descriptor_from_page_addr(virt_page_addr)?;

        if !page_desc.is_valid() {
            return Err("Page marked invalid");
        }

        page_desc.try_attributes()
    }

    fn try_virt_addr_to_phys_addr(
        &self,
        virt_addr: Address<Virtual>,
    ) -> Result<Address<Physical>, &'static str> {
        let virt_page = PageAddress::from(virt_addr.align_down_page());
        let phys_page = self.try_virt_page_addr_to_phys_page_addr(virt_page)?;

        Ok(phys_page.into_inner() + virt_addr.offset_into_page())
    }
}
