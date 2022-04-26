// Sparc = Daemon/Background process (Although prob better to be in userspace as a process)
// Not a 'service' as in syscall, but a 'service' as in a process that runs in kernel mode
// and acts like kernel modules that handle runtime code
// or separate threads that run specific kernel code to do things like filesystem management, network management
// Example services may manage IO for networking by looking at the socket request queue and directing each request, copying the buffers from user -> kernel buffers, calling driver code
// and usually have a higher API for internal kernel use and can be started/closed on demand to allow certain things like disk view/filesystem view when they start

pub trait Sparc {
    fn handle_req(spart_req: SparcReq);
    fn start();
    fn close();
}

#[derive(Debug, Clone, Copy)]
pub enum SparcStatus {
    FOREGROUND,
    BACKGROUND,
}

pub struct SparcDetails {
    priority: i32,
    ram_usage: u64,
    cpu_usage: u64,
    vram_usage: u64,
    gpu_usage: u64,
    status: SparcStatus,
}

impl SparcDetails {
    pub fn new() -> Self {
        Self {
            priority: 1,
            ram_usage: 0,
            cpu_usage: 0,
            vram_usage: 0,
            gpu_usage: 0,
            status: SparcStatus::BACKGROUND,
        }
    }
    pub fn status(&self) -> SparcStatus {
        self.status
    }
}

pub struct SparcReq;
