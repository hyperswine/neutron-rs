use super::Process;
use alloc::vec::Vec;

/// The default Scheduler API for Neutron
pub struct KScheduler {
    /// Could sort by id, name, space,
    pub processes: Vec<Process>,
}

impl KScheduler {
    pub fn new(processes: Vec<Process>) -> Self {
        Self { processes }
    }

    /// Schedule the next lot of KThreads
    pub fn tick(&mut self) {}
}

// NOTES:

// SMP Scheduling

// So you have m processes and n processors
// Some threads want to access resources like a PCIe device or DRAM, which are all the same distance

// NUMA Scheduling

// So you have m processes and n processors
// Some threads want to access resources which may be closer or farer away from a certain processor
// A thread that wants to access a certain node should be scheduled on a processor node nearer to the resource. Need some API in the driver or scheduler/interrupt handler that determines it

// USERSPACE Scheduling

// via something like schedulerd/spx:sched
// And kernel maps its privileged vmobject containing a list of processes into the proc addr space
