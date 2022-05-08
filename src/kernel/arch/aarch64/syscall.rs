// Define syscall ABI which kernel sets up in the interrupt table
// Assuming: Single user, etc. For multi user, use software

// Implements the standard syscalls in services::syscalls
// for aarch64

use crate::services::syscall::*;

// -----------
// COMMON
// -----------

// VECTOR HANDLER, from https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/11_exceptions_part1_groundwork

// TODO: can be used with include_str!("syscall.S")
core::arch::global_asm!(
    "
.macro CALL_WITH_CONTEXT handler
__vector_handler:
	sub	sp,  sp,  #16 * 17

	stp	x0,  x1,  [sp, #16 * 0]
	stp	x2,  x3,  [sp, #16 * 1]
	stp	x4,  x5,  [sp, #16 * 2]
	stp	x6,  x7,  [sp, #16 * 3]
	stp	x8,  x9,  [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12]
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]

	mrs	x1,  ELR_EL1
	mrs	x2,  SPSR_EL1
	mrs	x3,  ESR_EL1

	stp	lr,  x1,  [sp, #16 * 15]
	stp	x2,  x3,  [sp, #16 * 16]

	mov	x0,  sp

	bl	handler

	b	__exception_restore_context

.size	__vector_handler, . - __vector_handler
.type	__vector_handler, function
.endm
"
);

core::arch::global_asm!(
    "
.align 11

__exception_vector_start:

.org 0x000
	CALL_WITH_CONTEXT current_el0_synchronous
.org 0x080
	CALL_WITH_CONTEXT current_el0_irq
.org 0x100
	FIQ_SUSPEND
.org 0x180
	CALL_WITH_CONTEXT current_el0_serror

.org 0x200
	CALL_WITH_CONTEXT current_elx_synchronous
.org 0x280
	CALL_WITH_CONTEXT current_elx_irq
.org 0x300
	FIQ_SUSPEND
.org 0x380
	CALL_WITH_CONTEXT current_elx_serror
"
);

#[repr(C)]
pub struct ExceptionContext {
    
    gpr: [u64; 30],

    
    lr: u64,

    
    elr_el1: u64,
    // Saved program status.
    // spsr_el1: SpsrEL1,

    // Exception syndrome register.
    // esr_el1: EsrEL1,
}


fn default_exception_handler(exc: &ExceptionContext) {
    panic!("CPU Exception!\n");
}

#[no_mangle]
unsafe extern "C" fn current_elx_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}
