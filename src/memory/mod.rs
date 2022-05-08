// SOME LOW LEVEL PAGING SHOULD BE HANDLED BY THE ARCH AND IMPORTED HERE AS AN API

// ALLOC HANDLED BY THE KERNEL
pub mod alloc;
// Memory Management Unit
pub mod mmu;

use crate::{bsp, common};
use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, Sub},
};

pub trait AddressType: Copy + Clone + PartialOrd + PartialEq {}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Physical {}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Virtual {}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Address<ATYPE: AddressType> {
    value: usize,
    _address_type: PhantomData<fn() -> ATYPE>,
}

impl AddressType for Physical {}
impl AddressType for Virtual {}

impl<ATYPE: AddressType> Address<ATYPE> {
    pub const fn new(value: usize) -> Self {
        Self {
            value,
            _address_type: PhantomData,
        }
    }

    pub const fn as_usize(self) -> usize {
        self.value
    }

    #[must_use]
    pub const fn align_down_page(self) -> Self {
        let aligned = common::align_down(self.value, bsp::memory::mmu::KernelGranule::SIZE);

        Self::new(aligned)
    }

    #[must_use]
    pub const fn align_up_page(self) -> Self {
        let aligned = common::align_up(self.value, bsp::memory::mmu::KernelGranule::SIZE);

        Self::new(aligned)
    }

    pub const fn is_page_aligned(&self) -> bool {
        common::is_aligned(self.value, bsp::memory::mmu::KernelGranule::SIZE)
    }

    pub const fn offset_into_page(&self) -> usize {
        self.value & bsp::memory::mmu::KernelGranule::MASK
    }
}

impl<ATYPE: AddressType> Add<usize> for Address<ATYPE> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: usize) -> Self::Output {
        match self.value.checked_add(rhs) {
            None => panic!("Overflow on Address::add"),
            Some(x) => Self::new(x),
        }
    }
}

impl<ATYPE: AddressType> Sub<Address<ATYPE>> for Address<ATYPE> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Address<ATYPE>) -> Self::Output {
        match self.value.checked_sub(rhs.value) {
            None => panic!("Overflow on Address::sub"),
            Some(x) => Self::new(x),
        }
    }
}

impl fmt::Display for Address<Physical> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let q3: u8 = ((self.value >> 32) & 0xff) as u8;
        let q2: u16 = ((self.value >> 16) & 0xffff) as u16;
        let q1: u16 = (self.value & 0xffff) as u16;

        write!(f, "0x")?;
        write!(f, "{:02x}_", q3)?;
        write!(f, "{:04x}_", q2)?;
        write!(f, "{:04x}", q1)
    }
}

impl fmt::Display for Address<Virtual> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let q4: u16 = ((self.value >> 48) & 0xffff) as u16;
        let q3: u16 = ((self.value >> 32) & 0xffff) as u16;
        let q2: u16 = ((self.value >> 16) & 0xffff) as u16;
        let q1: u16 = (self.value & 0xffff) as u16;

        write!(f, "0x")?;
        write!(f, "{:04x}_", q4)?;
        write!(f, "{:04x}_", q3)?;
        write!(f, "{:04x}_", q2)?;
        write!(f, "{:04x}", q1)
    }
}
