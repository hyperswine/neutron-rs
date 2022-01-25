// single address space page -> frame mapping
// TODO: allocate a frame when needed and swap a frame to disk if needed (not too big of a deal with 64bit addressing but good to test)

struct FrameAddress {
    addr: u64
}

impl FrameAddress {
    fn all(&self) -> KVec<FrameAddress> {
        // return frames starting at 0x0 ... 0x80000000 incrementing by 4096 each time
    }
}

// stores 4096B of data as a block
// can access any byte with the index operator
struct Data;

// maybe not needed
pub struct Frame {
    start_addr: FrameAddress,
    used_space: u64,
    data: *mut Data,
}

// multiple address space -> default for aarch64 and riscv64
mod mas {
    // levels 1 to 4 page tables
    // allow loopbacks for quicker page -> frame translation
    pub struct L1PT;
    pub struct L2PT;
    pub struct L3PT;
    pub struct L4PT;
}

// single address space
mod sas {
    pub struct SingleAddressPT {
        n_frames: u64,
        n_free_frames: u64,
        frames: KVec<FrameAddress>,
    }
    
    const MAX_FRAMES_64b: Size = 32768;
    // kernel only needs 4 frames at boot
    const FRAMES_USED_KERN: Size = 4;
    
    impl SingleAddressPT {
        fn new() -> SingleAddressPT {
            SingleAddressPT{n_frames: MAX_FRAMES_64b, n_free_frames: MAX_FRAMES_64b - FRAMES_USED_KERN, frames: FrameAddress::all()}
        }
    }
    
}


