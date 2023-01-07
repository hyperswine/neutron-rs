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

    /// Schedule the next lot of KThreads. Register this to the interval timer interrupt
    pub fn tick(&mut self) {
        // randomised scheduling
    }
}

/// A threadqueue with a max of e.g. 1000 kthreads. Maybe impl From iter for it?
pub type ThreadQueue<const N: usize> = ([u64; N], [f32; N]);

// impl FromIterator for ThreadQueue {
// }

pub fn randomised_scheduling(threads: ThreadQueue<1000>) {
    // schedule each thread according to their priority and some RNG generation (hardware?)
}

#[test]
fn test_scheduler() {}

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

// extern "C" {
//     pub fn register_programmable_interrupt();
// }
