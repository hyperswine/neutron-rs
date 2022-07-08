---
layout: default
title: Linker
parent: Neutron
---

## Linkers

The linker is a great program to properly link objects together. Into their respective sections.

### Section Header

An ELF file has a list of sections in any specific order. So should we just scan through it line by line to know where everything is?

No. We dont have to. We could just have a section metadata area ("section header table"). That lists exactly where each section is in the file. What type of section that is. Its size, etc.

Section headers are needed for proper linking cause we need to link the same types of sections with each other into a bigger section.

```rust
data SectionHeader {
    name: [u8; name_siz]
    section_type: SectionType
    // 0xf0000000 for proc specific semantics
    // ... for a bunch of stuff
    // 00000100 for executable machine instructions (.text)
    // 00000010 if the section occupies memory during execution
    // 00000001 if the section contains writable data (.data)
    flags: u64
    // 0 if section doesnt appear in the memory img of process
    // basically the virtual addr of the data in memory
    // if the section is loaded
    addr: u64
    offset: u64
    size: u64
    // depends on the section type
    link: SectionLink
    // depends on the section type
    info: SectionInfo
    // basically addr % align = 0
    align: u64
    // if this section is something like a symbol table. Where you also have 'sections'. Then what is the entry size of that
    // otherwise 0, e.g. text
    entry_size: u64
}

enum SectionType u32 {
    Null
    ProgramData
    SymbolTable
    StringTable
    RelationEntriesWithAddends
    SymbolHashTable
    DynamicLinkInfo
    Notes
    NoData
    RelocationWithoutAddends
    Reserved
    ConstructorArrays
    DestructorArrays
    Grroup
    ExtendedSectionIndices
    NumberOfDefinedTypes
    OsSpecific
}

enum SectionInfo u32 {
    Null
    OneGreater
    IndexOfRelocation
}

enum SectionLink u32 {
    Undef
    // if it is an index, it is a u32 val
    Index => u32
}
```

### Program Header

The program header is needed for loading segments. It defines where segments should be loaded into RAM.

NOTE: we always assume 64 bits.

```rust
data ProgramHeader {
    program_type: u32
    // depends on the segment type
    // note on 32 bit, its actually later
    flags: SegmentFlags
    // offset of the segment in the file img
    offset: u64
    // virtual addr where segment should be loaded
    vaddr: u64
    // if not using virtual addressing (paging/segmentation), then can use this instead. Prob can just ignore for most part
    paddr: u64
    size: u64
    // size of segment in memory. Can be 0 like size
    // should prob just be size, though idk if compression, fragmentation and paging play a part
    memory_size: u64
    // 0/1 = no alignment. Otherwise should be a power of 2
    // vaddr = offset % align
    // that means it should be decided if align = 8 for example
    align: u64
}
```
