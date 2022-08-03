use goblin::{
    container::{Container, Ctx},
    elf::{program_header, Elf, ProgramHeader},
};
use core::{arch::asm, intrinsics::transmute};
use alloc::vec::Vec;
use log::info;

const ELF64_HDR_SIZE: usize = 64;
pub const USERSPACE_STACK_START: u64 = 0x0000_FFFF_FFFF_FFFF;

pub fn load_elf_userspace(elf_img_bytes: &[u8]) {
    let header =
        Elf::parse_header(&elf_img_bytes[..ELF64_HDR_SIZE]).map_err(|_| "parse elf header error");

    let header = match header {
        Ok(h) => h,
        Err(err) => panic!("Error! {err}"),
    };

    info!("header = {header:?}");

    let program_hdr_table_size = (header.e_phnum * header.e_phentsize) as usize;
    let program_table = &elf_img_bytes[ELF64_HDR_SIZE..ELF64_HDR_SIZE + program_hdr_table_size];

    let mut elf = Elf::lazy_parse(header)
        .map_err(|_| "cannot parse ELF file")
        .unwrap();

    let ctx = Ctx {
        le: scroll::Endian::Little,
        container: Container::Big,
    };

    // Parse and assemble the program headers

    elf.program_headers = match ProgramHeader::parse(
        &program_table,
        header.e_phoff as usize,
        header.e_phnum as usize,
        ctx,
    )
    .map_err(|_| "parse program headers error")
    {
        Ok(r) => r,
        Err(err) => panic!("Error! {err}"),
    };

    // Addresses should start with 0s for userspace images

    let sections: Vec<&[u8]> = elf
        .program_headers
        .iter()
        .map(|h| {
            if h.p_type == program_header::PT_INTERP && h.p_filesz != 0 {
                let offset = h.p_filesz as usize;
                let count = h.p_offset as usize;

                &elf_img_bytes[offset..offset + count]
            } else {
                &[0; 1]
            }
        })
        .collect();

    // Load sections

    elf.program_headers.iter().enumerate().for_each(|(ind, p)| {
        // map_segment(sections[ind], p.p_vaddr);
        info!("Found a segment! {p:?}");
    });

    // Create a process addr space struct in the kernel for the process
    // for arm: Set TTBR0 to a new page table for the process

    let elf_entry_addr = elf.header.e_entry as *const ();
    let entry = unsafe { transmute::<*const (), extern "C" fn()>(elf_entry_addr) };

    // arcboot api function
    // set_stack(USERSPACE_STACK_START);

    entry();
}
