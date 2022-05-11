// A record of mapped pages.

use crate::{
    memory::{Address, Physical, Virtual},
    types::{paging::*, synchronisation::InitStateLock},
};

#[derive(Copy, Clone)]
struct MappingRecordEntry {
    pub users: [Option<&'static str>; 5],
    pub phys_start_addr: Address<Physical>,
    pub virt_start_addr: Address<Virtual>,
    pub num_pages: usize,
    pub attribute_fields: AttributeFields,
}

struct MappingRecord {
    inner: [Option<MappingRecordEntry>; 12],
}

// ----------------
// Global instances
// ----------------

static KERNEL_MAPPING_RECORD: InitStateLock<MappingRecord> =
    InitStateLock::new(MappingRecord::new());

impl MappingRecordEntry {
    pub fn new(
        name: &'static str,
        virt_region: &MemoryRegion<Virtual>,
        phys_region: &MemoryRegion<Physical>,
        attr: &AttributeFields,
    ) -> Self {
        Self {
            users: [Some(name), None, None, None, None],
            phys_start_addr: phys_region.start_addr(),
            virt_start_addr: virt_region.start_addr(),
            num_pages: phys_region.num_pages(),
            attribute_fields: *attr,
        }
    }

    fn find_next_free_user(&mut self) -> Result<&mut Option<&'static str>, &'static str> {
        if let Some(x) = self.users.iter_mut().find(|x| x.is_none()) {
            return Ok(x);
        };

        Err("Storage for user info exhausted")
    }

    pub fn add_user(&mut self, user: &'static str) -> Result<(), &'static str> {
        let x = self.find_next_free_user()?;
        *x = Some(user);
        Ok(())
    }
}

impl MappingRecord {
    pub const fn new() -> Self {
        Self { inner: [None; 12] }
    }

    fn find_next_free(&mut self) -> Result<&mut Option<MappingRecordEntry>, &'static str> {
        if let Some(x) = self.inner.iter_mut().find(|x| x.is_none()) {
            return Ok(x);
        }

        Err("Storage for mapping info exhausted")
    }

    fn find_duplicate(
        &mut self,
        phys_region: &MemoryRegion<Physical>,
    ) -> Option<&mut MappingRecordEntry> {
        self.inner
            .iter_mut()
            .filter(|x| x.is_some())
            .map(|x| x.as_mut().unwrap())
            .filter(|x| x.attribute_fields.mem_attributes == MemAttributes::Device)
            .find(|x| {
                if x.phys_start_addr != phys_region.start_addr() {
                    return false;
                }

                if x.num_pages != phys_region.num_pages() {
                    return false;
                }

                true
            })
    }

    pub fn add(
        &mut self,
        name: &'static str,
        virt_region: &MemoryRegion<Virtual>,
        phys_region: &MemoryRegion<Physical>,
        attr: &AttributeFields,
    ) -> Result<(), &'static str> {
        let x = self.find_next_free()?;

        *x = Some(MappingRecordEntry::new(
            name,
            virt_region,
            phys_region,
            attr,
        ));
        Ok(())
    }
}

// ----------------
// Public Code
// ----------------

pub fn kernel_add(
    name: &'static str,
    virt_region: &MemoryRegion<Virtual>,
    phys_region: &MemoryRegion<Physical>,
    attr: &AttributeFields,
) -> Result<(), &'static str> {
    KERNEL_MAPPING_RECORD.write(|mr| mr.add(name, virt_region, phys_region, attr))
}

pub fn kernel_find_and_insert_mmio_duplicate(
    mmio_descriptor: &MMIODescriptor,
    new_user: &'static str,
) -> Option<Address<Virtual>> {
    let phys_region: MemoryRegion<Physical> = (*mmio_descriptor).into();

    KERNEL_MAPPING_RECORD.write(|mr| {
        let dup = mr.find_duplicate(&phys_region)?;

        if let Err(x) = dup.add_user(new_user) {
            // warn!("{}", x);
        }

        Some(dup.virt_start_addr)
    })
}
