// pub mod manager;
pub mod elf;
pub mod scheduler;

use crate::types::KPriorityQueue;
use lazy_static::lazy_static;

use crate::alloc::{vec, vec::Vec};

type ThreadID = u64;

lazy_static! {
    pub static ref THREAD_POOL: Vec<ThreadID> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

pub struct Process {
    priority: i64,
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
    pub fn new(_prio: i64) -> Self {
        Self { priority: _prio }
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
    Process::new(0);
    std::println!("Process succesfully created!");
}