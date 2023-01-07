// ------------
// AddressTypes
// ------------

// * Alot of these things require KernelGranule and other Pi 4B specific types

impl<ATYPE: AddressType> Step for PageAddress<ATYPE> {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start > end {
            return None;
        }

        // Since start <= end, do unchecked arithmetic.
        Some((end.inner.as_usize() - start.inner.as_usize()) >> KernelGranule::SHIFT)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        start.checked_offset(count as isize)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        start.checked_offset(-(count as isize))
    }
}

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

        let delta = count.unsigned_abs().checked_mul(KernelGranule::SIZE)?;
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

// Implementations

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
        let aligned = align_down(self.value, KernelGranule::SIZE);

        Self::new(aligned)
    }

    #[must_use]
    pub const fn align_up_page(self) -> Self {
        let aligned = align_up(self.value, KernelGranule::SIZE);

        Self::new(aligned)
    }

    pub const fn is_page_aligned(&self) -> bool {
        is_aligned(self.value, KernelGranule::SIZE)
    }

    pub const fn offset_into_page(&self) -> usize {
        self.value & KernelGranule::MASK
    }
}

// Translation Tables

pub type KernelTranslationTable =
    <KernelVirtAddrSpace as AssociatedTranslationTable>::TableStartFromTop;

pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;

pub type KernelVirtAddrSpace = AddressSpace<{ kernel_virt_addr_space_size() }>;

pub fn kernel_translation_tables() -> &'static InitStateLock<KernelTranslationTable> {
    &KERNEL_TABLES
}

pub fn virt_mmio_remap_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(mmio_remap_size());

    let start_page_addr = virt_mmio_remap_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

pub fn kernel_add_mapping_records_for_precomputed() {
    let virt_code_region = virt_code_region();
    kernel_add_mapping_record(
        "Kernel code and RO data",
        &virt_code_region,
        &kernel_virt_to_phys_region(virt_code_region),
        &kernel_page_attributes(virt_code_region.start_page_addr()),
    );

    let virt_data_region = virt_data_region();
    kernel_add_mapping_record(
        "Kernel data and bss",
        &virt_data_region,
        &kernel_virt_to_phys_region(virt_data_region),
        &kernel_page_attributes(virt_data_region.start_page_addr()),
    );

    let virt_boot_core_stack_region = virt_boot_core_stack_region();
    kernel_add_mapping_record(
        "Kernel boot-core stack",
        &virt_boot_core_stack_region,
        &kernel_virt_to_phys_region(virt_boot_core_stack_region),
        &kernel_page_attributes(virt_boot_core_stack_region.start_page_addr()),
    );
}

// Pi 4 Memory Setup

#[link_section = ".data"]
#[no_mangle]
static KERNEL_TABLES: InitStateLock<KernelTranslationTable> =
    InitStateLock::new(KernelTranslationTable::new_for_precompute());

#[link_section = ".text._start_arguments"]
#[no_mangle]
static PHYS_KERNEL_TABLES_BASE_ADDR: u64 = 0xCCCCAAAAFFFFEEEE;

// ----------
// Private Code for Setup
// ----------

const fn kernel_virt_addr_space_size() -> usize {
    let __kernel_virt_addr_space_size;

    // apparently needed for some reason
    include!("kernel_virt_addr_space_size.ld");

    __kernel_virt_addr_space_size
}

const fn size_to_num_pages(size: usize) -> usize {
    assert!(size > 0);
    assert!(size % KernelGranule::SIZE == 0);

    size >> KernelGranule::SHIFT
}

fn virt_code_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(code_size());

    let start_page_addr = virt_code_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

fn virt_data_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(data_size());

    let start_page_addr = virt_data_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

fn virt_boot_core_stack_region() -> MemoryRegion<Virtual> {
    let num_pages = size_to_num_pages(boot_core_stack_size());

    let start_page_addr = virt_boot_core_stack_start();
    let end_exclusive_page_addr = start_page_addr.checked_offset(num_pages as isize).unwrap();

    MemoryRegion::new(start_page_addr, end_exclusive_page_addr)
}

fn kernel_virt_to_phys_region(virt_region: MemoryRegion<Virtual>) -> MemoryRegion<Physical> {
    let phys_start_page_addr =
        try_kernel_virt_page_addr_to_phys_page_addr(virt_region.start_page_addr()).unwrap();

    let phys_end_exclusive_page_addr = phys_start_page_addr
        .checked_offset(virt_region.num_pages() as isize)
        .unwrap();

    MemoryRegion::new(phys_start_page_addr, phys_end_exclusive_page_addr)
}

fn kernel_page_attributes(virt_page_addr: PageAddress<Virtual>) -> AttributeFields {
    try_kernel_page_attributes(virt_page_addr).unwrap()
}

// -----------------
// CONSTANTS & MMIO
// -----------------

use core::{cell::UnsafeCell, iter::Step, marker::PhantomData};

use crate::{
    drivers::arm::common,
    types::{align_down, align_up, is_aligned, synchronisation::InitStateLock, paging::{PageAddress, MemoryRegion, AttributeFields}}, memory::{AddressType, Address, mmu::{AssociatedTranslationTable, TranslationGranule, AddressSpace, kernel_add_mapping_record, try_kernel_page_attributes, try_kernel_virt_page_addr_to_phys_page_addr}, Virtual, Physical},
};

// Symbols from the linker script.
extern "Rust" {
    static __code_start: UnsafeCell<()>;
    static __code_end_exclusive: UnsafeCell<()>;

    static __data_start: UnsafeCell<()>;
    static __data_end_exclusive: UnsafeCell<()>;

    static __mmio_remap_start: UnsafeCell<()>;
    static __mmio_remap_end_exclusive: UnsafeCell<()>;

    static __boot_core_stack_start: UnsafeCell<()>;
    static __boot_core_stack_end_exclusive: UnsafeCell<()>;
}

// MMIO Addresses for GPIO, UART, GIC
pub mod mmio {
    use crate::memory::{Address, Physical};

    pub const GPIO_START: Address<Physical> = Address::new(0xFE20_0000);
    pub const GPIO_SIZE: usize = 0xA0;

    pub const PL011_UART_START: Address<Physical> = Address::new(0xFE20_1000);
    pub const PL011_UART_SIZE: usize = 0x48;

    pub const GICD_START: Address<Physical> = Address::new(0xFF84_1000);
    pub const GICD_SIZE: usize = 0x824;

    pub const GICC_START: Address<Physical> = Address::new(0xFF84_2000);
    pub const GICC_SIZE: usize = 0x14;

    pub const END: Address<Physical> = Address::new(0xFF85_0000);
}

pub const END: Address<Physical> = mmio::END;

#[inline(always)]
fn virt_code_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __code_start.get() as usize })
}

#[inline(always)]
fn code_size() -> usize {
    unsafe { (__code_end_exclusive.get() as usize) - (__code_start.get() as usize) }
}

#[inline(always)]
fn virt_data_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __data_start.get() as usize })
}

#[inline(always)]
fn data_size() -> usize {
    unsafe { (__data_end_exclusive.get() as usize) - (__data_start.get() as usize) }
}

#[inline(always)]
fn virt_mmio_remap_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __mmio_remap_start.get() as usize })
}

#[inline(always)]
fn mmio_remap_size() -> usize {
    unsafe { (__mmio_remap_end_exclusive.get() as usize) - (__mmio_remap_start.get() as usize) }
}

#[inline(always)]
fn virt_boot_core_stack_start() -> PageAddress<Virtual> {
    PageAddress::from(unsafe { __boot_core_stack_start.get() as usize })
}

#[inline(always)]
fn boot_core_stack_size() -> usize {
    unsafe {
        (__boot_core_stack_end_exclusive.get() as usize) - (__boot_core_stack_start.get() as usize)
    }
}

#[inline(always)]
pub fn phys_addr_space_end_exclusive_addr() -> PageAddress<Physical> {
    PageAddress::from(END)
}
