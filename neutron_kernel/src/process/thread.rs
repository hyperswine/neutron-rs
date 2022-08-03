use alloc::vec::Vec;
use lazy_static::lazy_static;

pub type KThreadID = u64;
pub type TaskID = usize;

pub struct ExecutionContext {
    // registers like x0-31 and special ones like sp
    general_reg: [u64; 32],
    sp: u64,
}

pub struct KThread {
    id: KThreadID,
    context: ExecutionContext,
}
