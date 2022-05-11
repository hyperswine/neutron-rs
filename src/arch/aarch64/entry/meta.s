; store any metadata here, like multiboot headers
; TODO: can make this a packed struct in rust
; with #[link_section = ".text"] and ALIGN(8) 8 Bytes for .text in linker
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
