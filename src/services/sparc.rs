// Sparc = Service
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
