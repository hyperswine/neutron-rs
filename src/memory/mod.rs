// SOME LOW LEVEL PAGING SHOULD BE HANDLED BY THE ARCH AND IMPORTED HERE AS AN API

pub mod alloc;
pub mod mmu;

use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, Sub},
};

// -----------------
// General Address API
// -----------------

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

// -----------------
// FORMATTED OUTPUT
// -----------------

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
