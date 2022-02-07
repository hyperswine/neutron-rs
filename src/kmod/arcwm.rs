// WINDOW SERVER LIBRARY THAT LINKS WITH NEUTRON CORE

use crate::services::sparc::{Sparc, SparcDetails, SparcReq};

enum WindowState {
    MINIMIZED,
    FOREGROUND,
    BACKGROUND,
}

struct ArcWin {
    sparc_details: SparcDetails,
    state: WindowState,
    z_index: i32,
    position: [f32; 2],
    responsive: bool,
}

impl ArcWin {
    fn new() -> Self {
        Self {
            sparc_details: SparcDetails::new(),
            state: WindowState::FOREGROUND,
            z_index: DEFAULT_Z_INDEX,
            position: [(SCR_WIDTH / 2) as f32, (SCR_HEIGHT / 2) as f32],
            responsive: true,
        }
    }
}

impl Sparc for ArcWin {
    // observer pattern
    fn handle_req(sparc_req: SparcReq) {}

    fn start() {}

    fn close() {}
}

const DEFAULT_Z_INDEX: i32 = 100;
const SCR_HEIGHT: i32 = 1080;
const SCR_WIDTH: i32 = 1920;

#[test]
fn test_basics_sparc() {
    let _sparc = ArcWin {
        sparc_details: SparcDetails::new(),
        state: WindowState::FOREGROUND,
        z_index: DEFAULT_Z_INDEX,
        position: [(SCR_WIDTH / 2) as f32, (SCR_HEIGHT / 2) as f32],
        responsive: true,
    };
}
