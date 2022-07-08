---
layout: default
title: Architectures
parent: Updates
---

## RISC V

### SBI Calls

For riscv, neutron interacts with sbi via the `ecall` instruction. Like so:

```rust
unsafe { asm!(
            "ecall",
            in("a0") arg0, in("a1") arg1,
            in("a6") function, in("a7") extension,
            lateout("a0") error, lateout("a1") value,
        ) }
```

- this uses the values `arg0, arg1, function, extension, error, value`

## AARCH64

```asm
# AARCH64
# FINDING CURR EXCEPTION LEVEL

.globl get_el
get_el:
    mrs x0, CurrentEL
    lsr x0, x0, #2
    ret

# CHANGING EXCEPTION LEVEL

master:
    # disabled if page tables dont exist yet, e.g. before bootloader sets it up
    ldr    x0, =SCTLR_VALUE_MMU_DISABLED
    msr    sctlr_el1, x0        

    # hypervisor reg. Technically dont need but should have
    ldr    x0, =HCR_VALUE
    msr    hcr_el2, x0

    # security register
    ldr    x0, =SCR_VALUE
    msr    scr_el3, x0

    # saved program status reg. Execution level should be el3
    ldr    x0, =SPSR_VALUE
    msr    spsr_el3, x0

    # return to this addr
    adr    x0, el1_entry        
    msr    elr_el3, x0

    eret                
```

<https://developer.arm.com/documentation/den0024/a/The-Memory-Management-Unit/Translation-tables-in-ARMv8-A>
<https://wiki.osdev.org/ARM_Paging>

armv8-a uses the long descriptor format (64 bit)

armv7-a uses short descriptors and the ID_MMFR0 register
which stores a bunch of flags on PXN (privilege-execute-never) and etc
TTBCR -> for controlling
TTBR0 -> base addr of TTB0 and etc
TTBR1 -> base addr of TTB1 and etc
