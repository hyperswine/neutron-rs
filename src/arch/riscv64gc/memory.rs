pub struct SingleAddressPT {
    n_frames: u64,
    n_free_frames: u64,
    frames: Vec<FrameAddress>,
}

const MAX_FRAMES_64b: usize = 32768;
// kernel only needs 4 frames at boot
const FRAMES_USED_KERN: usize = 4;

impl SingleAddressPT {
    fn new() -> SingleAddressPT {
        SingleAddressPT {
            n_frames: MAX_FRAMES_64b,
            n_free_frames: MAX_FRAMES_64b - FRAMES_USED_KERN,
            frames: FrameAddress::all(),
        }
    }
}
