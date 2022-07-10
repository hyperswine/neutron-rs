// -----------------
// SYSCALL NUMBERS
// -----------------

// Syscalls will be different on each arch. They should be handled in arch/syscall at first, which then calls handle_syscall()
// This is place is strictly arch independent code
// Syscalls are usually grouped into "creation", "management" and "destruction". Destruction syscalls are either decrement or total kill type syscalls

// A task == process
// It can have a parent and set of children

// Syscalls are designed to be small and built up upon by userspace libraries like NeutronAPI and services like spx:fs that other processes should subscribe to
// Most non-service userspace libs should not be using syscalls directly, but rather NeutronAPI and other libs like Arc and Dioxus. Rust std, tokio, and other frameworks should be used as well

// IPC
// Neutron supports block oriented or stream oriented IPC
// At the end of the day, all you are really doing is writing to some VMO with or without CoW semantics and transferring that to another process, who may then transfer it again, write to it, or write to hardware

use crate::process::TaskID;

pub type Handle = usize;

pub enum Syscall {
    // Handles
    Close,
    Duplicate,
    Read,
    Write,

    // Files
    FileOpen,
    FileCreate,
    FileDelete,

    // Objects
    ObjectInfo,
    ObjectSignal,
    ObjectWait,

    // Memory
    VMOCreate,
    VMOTransfer,
    VMOMap,

    // Tasks
    TaskSpawn,
    TaskSuspend,
    TaskKill,
    TaskProfile,
    // User-kernel threads
    ThreadCreate,
    ThreadKill,

    // IPC
    PortCreate,
    PortClose,
}

pub fn handle_syscall(syscall_number: Syscall) {}

// Decrement the ref count by 1 for that object
fn handle_close() {}

fn task_spawn() -> TaskID {
    1
}

// Change priority or parent or children
fn task_profile() {}

// Kill overrides reference counting of objects or just decrements it
// Technically a kobject has 1 or more khandles, but should just be 1 here
// Of which the khandle belongs to its parent
fn task_kill() {}
