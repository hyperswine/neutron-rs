ENTRY(_multiboot_entry)

/* TODO: setup the right headers
 also in main.rs */

SECTIONS
{
	. = 1M;

    .boot : { *(.multiboot_header) }

	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) }
	. = ALIGN(8);
	. = . + 0x1000; /* 4kB of stack memory */
	stack_top = .;
}
