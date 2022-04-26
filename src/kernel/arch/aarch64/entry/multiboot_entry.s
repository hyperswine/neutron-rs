# SHOULD BE PLACED within 8KB of the start of the image, no dupes
# Possible to label it as its own section
.section .multiboot_header

header_start:
    # magic number (multiboot 2)
    .quad 0xe85250d6   

    # architecture 1 (protected mode aarch64)
    .quad 0

    # header length
    .quad header_end - header_start

    # checksum -> bootloader should verify by inverse op
    .quad 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    # insert optional multiboot tags here

    # END TAG
    # type
    .word 0
    # flags
    .word 0
    # size
    .quad 8
header_end:

# PROGRAM HEADER SHOULD BE AUTO GENERATED FROM THE SECTIONS

# SECTIONS:
#
# Reference: https://developer.arm.com/documentation/100068/0608/migrating-from-armasm-to-the-armclang-integrated-assembler/sections
#
# CD (code)
# SS (stack)
# DS (data)
# ES (extra data)

# Setup code segment
.section .text

_multiboot_entry:
    ldr x30, =stack_top
    mov sp, x30
    bl _start
    bl .

# Setup stack segment
.section .stack

# Setup data segment
.section .data

# SECTION HEADER SHOULD BE AUTO GENERATED FROM THE SECTIONS
