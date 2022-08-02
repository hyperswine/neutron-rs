// pub mod manager;
pub mod elf;
pub mod scheduler;

use crate::types::KPriorityQueue;
use lazy_static::lazy_static;

use alloc::{vec, vec::Vec, string::String};

pub type ThreadID = u64;
pub type TaskID = usize;

lazy_static! {
    pub static ref THREAD_POOL: Vec<ThreadID> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

pub struct ProcessManager {
    processes: Vec<Process>, //could sort by id, name, space,
}

pub struct Process {
    id: u32,
    name: String,
    space_allocated: u32,
    space_used: u32,
    status: ProcessStatus,
    priority: i64,
}

// Kind of like Sparc but not really
pub enum ProcessStatus {
    UP,
    DOWN,
    BLOCKED,
}

pub enum ProcessPrivilege {
    FULL,
    RD_ONLY,
    RD_WRITE,
    NONE,
}

pub struct ElfBinary;

pub enum ProcessExitStatus {
    SUCCESS,
    BAD,
    PANICKED,
}

impl Process {
    pub fn new(
        id: u32,
        name: String,
        space_allocated: u32,
        space_used: u32,
        status: ProcessStatus,
        priority: i64,
    ) -> Self {
        Self {
            id,
            name,
            space_allocated,
            space_used,
            status,
            priority,
        }
    }

    /// A process has 5 regions, https://en.wikipedia.org/wiki/File:Program_memory_layout.pdf
    /// Should return process exit code
    pub fn execute_elf64(&self, validated_elf_bin: &ElfBinary) -> ProcessExitStatus {
        

        ProcessExitStatus::SUCCESS
    }
    // callable by kernel only, when userspace process calls thread.create() or any async/await code that generates a new std::thread
    pub fn create_thread(&self) {
        // create another software thread from the thread pool
        // and queue for execution based on process priority (kernel priority queue / KPrioQueue)
        // THREAD_POOL.
    }
}

// -------------
// TEST
// -------------

#[test]
fn test_process() {
    // Process::new(0);
    std::println!("Process succesfully created!");
}
