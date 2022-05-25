// DRIVER SUITE FOR SPECTRAL SOC
// ASSUME BOARD FIRMWARE HAS SET UP MMIO AT CERTAIN POINTS, RIGHT AFTER BOARD IS BOOTED AND FIRMWARE HAS BEEN EXECUTED
// ACPI PROVIDES ACCESS TO A DEVICE TREEE/TABLE AT MEMORY ADDRESSES. BASICALLY A BUNCH OF FUNCTIONS YOU CAN JUMP/CALL TO SEE THE DEVICES AND INTERACT WITH THEM
// NOTE: Little Endian everywhere

// ALSO INVOLVES NICE POWER MANAGEMENT (P0-Pn) AND SLEEP (S0-S5)
// https://en.wikipedia.org/wiki/Advanced_Host_Controller_Interface
// https://www.sifive.com/blog/risc-v-qemu-part-1-privileged-isa-hifive1-virtio

// ADDRESSES
// UART 0x10 000 000
// MMIO 0x10 008 000                (+4096)
// MMIO 0x10 007 000                (+4096)
// MMIO 0x10 006 000                (+4096)

// EXAMPLE SIFIVE
// Device Tree Struct
// magic:        0xd00dfeed
// totalsize:        0x10000 (65536)
// off_dt_struct:    0x40
// off_dt_strings:    0x868
// off_mem_rsvmap:    0x30
// version:        17
// last_comp_version:    16
// boot_cpuid_phys:    0x0
// size_dt_strings:    0x129
// size_dt_struct:    0x828
