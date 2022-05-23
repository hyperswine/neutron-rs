ENTRY(_setup)

SECTIONS
{
	/* should start at 1MB for multiboot */
	/* 0x100000 = 1MiB */
	/* GONNA DO 0x4M for now */
	. = 0x40000000;

    /* .multiboot : { *(.multiboot_header) } */
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss COMMON) }

	. = ALIGN(8);
	. = . + 0x1000; /* 4kB of stack memory */
	stack_top = .;
}
