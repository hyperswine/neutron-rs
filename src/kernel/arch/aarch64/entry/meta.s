; store any metadata here, like multiboot headers
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
