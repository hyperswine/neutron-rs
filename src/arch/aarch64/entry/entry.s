; the actual entry for neutron_kernel. As specified by entry/multiboot_entry.x
; should be in arcboot
.global _setup
_setup:
    // Only proceed if the core executes in EL2. Loop otherwise.
	mrs	x0, CurrentEL
	cmp	x0, {CONST_CURRENTEL_EL2}
	b.ne	.L_parking_loop

	// Only proceed on the boot core. Loop otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, {CONST_CORE_ID_MASK}
	ldr	x2, BOOT_CORE_ID
	cmp	x1, x2
	b.ne	.L_parking_loop

	// If execution reaches here, it is the boot core.

	// Initialize DRAM. Setup stack at the end of bss (kernel semantics)
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive

.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

.L_prepare_rust:
	// Set the stack pointer. This ensures that any code in EL2 that needs the stack will work.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	// Jump to Rust code. x0 holds the function argument provided to _start_rust.
	b	_start_rust

.L_parking_loop:
	wfe
	b	.L_parking_loop