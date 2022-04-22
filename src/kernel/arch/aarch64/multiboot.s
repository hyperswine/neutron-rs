.section multiboot_header

# before the ELF header, place header_start. Note, doubles are 32bits
header_start:
    # magic number (multiboot 2)
    .double 0xe85250d6   

    # architecture 1 (protected mode aarch64)
    .double 0

    # header length
    .double header_end - header_start

    # checksum -> bootloader should verify by inverse op
    .double 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    # insert optional multiboot tags here

    # END TAG
    # type
    .word 0
    # flags
    .word 0
    # size
    .double 8

header_end:
