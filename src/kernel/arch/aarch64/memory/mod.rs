pub mod allocator;
pub mod virtual_memory;

// ------------
// TRANSLATION TABLES
// -------------

// https://developer.arm.com/documentation/den0024/a/The-Memory-Management-Unit/Translation-tables-in-ARMv8-A
// https://wiki.osdev.org/ARM_Paging

// armv8-a uses the long descriptor format (64 bit)

// armv7-a uses short descriptors and the ID_MMFR0 register
// which stores a bunch of flags on PXN (privilege-execute-never) and etc
// TTBCR -> for controlling
// TTBR0 -> base addr of TTB0 and etc
// TTBR1 -> base addr of TTB1 and etc

