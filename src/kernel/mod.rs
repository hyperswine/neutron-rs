// INTERFACE TO HANDLE DEPENDENT + INDEPENDENT CODE TOGETHER

// --------------------
// ARCH DEPENDENT STUFF
// --------------------

pub mod acpi;
pub mod arch;
#[cfg(feature = "posix")]
pub mod posix;

// --------------------
// NON-ARCH DEPENDENT STUFF
// --------------------

/*
AARCH64
FINDING CURR EXCEPTION LEVEL

.globl get_el
get_el:
    mrs x0, CurrentEL
    lsr x0, x0, #2
    ret

CHANGING EXCEPTION LEVEL

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
*/

use alloc::vec;

use crate::filesystem::hfs_v1::{File, Filesystem};

pub struct KernelManager {
    filesystem: Filesystem,
}

// Pass off to /sys/init. Make sure it exists. If it doesnt the whole thing should loop and show the error with a 10 second shutdown
impl KernelManager {
    pub fn kernel_manager_entry(&self) -> ! {
        loop {}
    }

    // basically init_entry. Set execution to ring 3/user mode
    pub fn init(&self) -> ! {
        // VFS load_process() into memory
        // let pid = load_process("/sys/init")
        // cpu.set_mode(U_MODE)
        // this_thread.process_run(pid)
        //  /sys/init should be able to spawn more threads using std library/neutron syscalls
        //  and run in U mode

        loop {}
    }

    // create a default Kernel Manager with a single empty file (dir) in the HFS
    pub fn new() -> KernelManager {
        KernelManager {
            filesystem: Filesystem::new(),
        }
    }
}

// --------------------
// TESTS
// --------------------

#[test_case]
fn test_kern_basics() {
    let _kern = KernelManager::new();
}
