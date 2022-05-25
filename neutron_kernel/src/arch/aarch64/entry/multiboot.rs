// -----------------
// MULTIBOOT
// -----------------

/*
.section .multiboot_header
header_start:
    .quad 0xe85250d6
    .quad 0
    .quad header_end - header_start
    .quad 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))
    .word 0
    .word 0
    .quad 8
header_end:
*/

struct MultibootHeaderSpec2 {
    magic: u128,
    zero: u128,
    size: u128,
    align_constant: u128,
    zero_two: u32,
    zero_three: u32,
    eight: u128,
}

const NEUTRON_MULTIBOOT_HEADER: MultibootHeaderSpec = MultibootHeaderSpec2 { magic: 0xe85250d6 };
