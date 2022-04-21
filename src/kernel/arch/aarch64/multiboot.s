.section multiboot_header

; before the ELF header, place header_start
header_start:
    .double 0xe85250d6                ; magic number (multiboot 2)
    .double 0                         ; architecture 0 (protected mode i386)
    .double header_end - header_start ; header length
    ; checksum -> bootloader should verify by inverse op
    .double 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    .word 0    ; type
    .word 0    ; flags
    .double 8    ; size

header_end:
