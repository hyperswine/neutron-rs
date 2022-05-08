// Useful Types
// TODO: FOR BSP, mostly just needs to link in a non bad way
// [cfg(target_arch = "aarch64")]
// use crate::drivers::arm::common

use crate::{
    memory::{Address, AddressType, Physical},
};
use core::{convert::From, iter::Step, num::NonZeroUsize, ops::Range};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct PageAddress<ATYPE: AddressType> {
    inner: Address<ATYPE>,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct MemoryRegion<ATYPE: AddressType> {
    start: PageAddress<ATYPE>,
    end_exclusive: PageAddress<ATYPE>,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum MemAttributes {
    CacheableDRAM,
    Device,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum AccessPermissions {
    ReadOnly,
    ReadWrite,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct AttributeFields {
    pub mem_attributes: MemAttributes,
    pub acc_perms: AccessPermissions,
    pub execute_never: bool,
}

#[derive(Copy, Clone)]
pub struct MMIODescriptor {
    start_addr: Address<Physical>,
    end_addr_exclusive: Address<Physical>,
}


//---------------
// PageAddress
//---------------

impl<ATYPE: AddressType> PageAddress<ATYPE> {
    pub const MAX: Self = PageAddress {
        inner: Address::new(usize::MAX).align_down_page(),
    };

    pub fn into_inner(self) -> Address<ATYPE> {
        self.inner
    }

    pub fn checked_offset(self, count: isize) -> Option<Self> {
        if count == 0 {
            return Some(self);
        }

        let delta = count
            .unsigned_abs()
            .checked_mul(bsp::memory::mmu::KernelGranule::SIZE)?;
        let result = if count.is_positive() {
            self.inner.as_usize().checked_add(delta)?
        } else {
            self.inner.as_usize().checked_sub(delta)?
        };

        Some(Self {
            inner: Address::new(result),
        })
    }
}

impl<ATYPE: AddressType> From<usize> for PageAddress<ATYPE> {
    fn from(addr: usize) -> Self {
        assert!(
            common::is_aligned(addr, bsp::memory::mmu::KernelGranule::SIZE),
            "Input usize not page aligned"
        );

        Self {
            inner: Address::new(addr),
        }
    }
}

impl<ATYPE: AddressType> From<Address<ATYPE>> for PageAddress<ATYPE> {
    fn from(addr: Address<ATYPE>) -> Self {
        assert!(addr.is_page_aligned(), "Input Address not page aligned");

        Self { inner: addr }
    }
}

impl<ATYPE: AddressType> Step for PageAddress<ATYPE> {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start > end {
            return None;
        }

        // Since start <= end, do unchecked arithmetic.
        Some(
            (end.inner.as_usize() - start.inner.as_usize())
                >> bsp::memory::mmu::KernelGranule::SHIFT,
        )
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start.checked_offset(count as isize)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start.checked_offset(-(count as isize))
    }
}

//----------------
// Memory Region
//----------------

impl<ATYPE: AddressType> MemoryRegion<ATYPE> {
    pub fn new(start: PageAddress<ATYPE>, end_exclusive: PageAddress<ATYPE>) -> Self {
        assert!(start <= end_exclusive);

        Self {
            start,
            end_exclusive,
        }
    }

    fn as_range(&self) -> Range<PageAddress<ATYPE>> {
        self.into_iter()
    }

    pub fn start_page_addr(&self) -> PageAddress<ATYPE> {
        self.start
    }

    pub fn start_addr(&self) -> Address<ATYPE> {
        self.start.into_inner()
    }

    pub fn end_exclusive_page_addr(&self) -> PageAddress<ATYPE> {
        self.end_exclusive
    }

    pub fn end_inclusive_page_addr(&self) -> PageAddress<ATYPE> {
        self.end_exclusive.checked_offset(-1).unwrap()
    }

    pub fn contains(&self, addr: Address<ATYPE>) -> bool {
        let page_addr = PageAddress::from(addr.align_down_page());
        self.as_range().contains(&page_addr)
    }

    pub fn overlaps(&self, other_region: &Self) -> bool {
        let self_range = self.as_range();

        self_range.contains(&other_region.start_page_addr())
            || self_range.contains(&other_region.end_inclusive_page_addr())
    }

    pub fn num_pages(&self) -> usize {
        PageAddress::steps_between(&self.start, &self.end_exclusive).unwrap()
    }

    pub fn size(&self) -> usize {
        let end_exclusive = self.end_exclusive.into_inner().as_usize();
        let start = self.start.into_inner().as_usize();

        end_exclusive - start
    }

    pub fn take_first_n_pages(&mut self, num_pages: NonZeroUsize) -> Result<Self, &'static str> {
        let count: usize = num_pages.into();

        let left_end_exclusive = self.start.checked_offset(count as isize);
        let left_end_exclusive = match left_end_exclusive {
            None => return Err("Overflow while calculating left_end_exclusive"),
            Some(x) => x,
        };

        if left_end_exclusive > self.end_exclusive {
            return Err("Not enough free pages");
        }

        let allocation = Self {
            start: self.start,
            end_exclusive: left_end_exclusive,
        };
        self.start = left_end_exclusive;

        Ok(allocation)
    }
}

impl<ATYPE: AddressType> IntoIterator for MemoryRegion<ATYPE> {
    type Item = PageAddress<ATYPE>;
    type IntoIter = Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Range {
            start: self.start,
            end: self.end_exclusive,
        }
    }
}

impl From<MMIODescriptor> for MemoryRegion<Physical> {
    fn from(desc: MMIODescriptor) -> Self {
        let start = PageAddress::from(desc.start_addr.align_down_page());
        let end_exclusive = PageAddress::from(desc.end_addr_exclusive().align_up_page());

        Self {
            start,
            end_exclusive,
        }
    }
}

//------------------
// MMIODescriptor
//------------------

impl MMIODescriptor {
    pub const fn new(start_addr: Address<Physical>, size: usize) -> Self {
        assert!(size > 0);
        let end_addr_exclusive = Address::new(start_addr.as_usize() + size);

        Self {
            start_addr,
            end_addr_exclusive,
        }
    }

    pub const fn start_addr(&self) -> Address<Physical> {
        self.start_addr
    }

    pub fn end_addr_exclusive(&self) -> Address<Physical> {
        self.end_addr_exclusive
    }
}
