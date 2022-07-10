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

    // NOTE: a process has 5 regions, https://en.wikipedia.org/wiki/File:Program_memory_layout.pdf
    // Should return process exit code
    pub fn execute_elf64(&self, validated_elf_bin: &ElfBinary) -> ProcessExitStatus {
        // load regions, request_pages is basically kmalloc()

        // let text_pages = request_pages(validated_elf_bin.text_region_size())
        // * Bss = uninitialised data, which should be in a bss region
        // Bss is only good for reducing file size
        // let bss_pages = request_pages(validated_elf_bin.bss_region_size())
        // Data means global vars and static vars (local + global) which should be compiled & mangled to vars in .data
        // let data_pages = request_pages(validated_elf_bin.data_region_size())
        // let stack_pages = request_pages(5 * self.priority)
        // let heap_pages = request_pages(5 * self.priority)

        // ! when a process under userspace code control wants more memory, they must instead call malloc() rather than kmalloc()
        // give control to the elf code, start at the top of the text segment
        // this_thread.execute(USER_MODE, text_pages)

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
