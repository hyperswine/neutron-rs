ENTRY(start);
 
/* Using 32bit addr space */
. = 0x80000000;

/* RESOURCES ON LINKER SCRIPTS
	https://web.mit.edu/rhel-doc/3/rhel-ld-en-3/scripts.html
	https://wiki.osdev.org/Linker_Scripts - Recommended
	https://home.cs.colorado.edu/~main/cs1300/doc/gnu/ld_3.html
 */

/* Small ELF Loader */
SECTIONS {
	/* Include entry point at start of binary */
	.text : ALIGN(4K) {
		*(.init);
		*(.text);
	}
	.bss : ALIGN(4K) {
		PROVIDE(bss_start = .);
		*(.bss);
		. += 4096;
		PROVIDE(stack_top = .);
		. += 4096;
		PROVIDE(global_pointer = .);
		PROVIDE(bss_end = .);
	}
	.rodata : ALIGN(4K) {
		*(.rodata);
	}
	.data : ALIGN(4K) {
		*(.data);
	}
	/* No heap for the bootloader and kernel. Kernel must set up paging
		and frames from the bootloader. */
}

/* Can specify elf64-little */
