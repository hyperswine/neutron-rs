// WINDOW SERVER LIBRARY THAT LINKS WITH NEUTRON CORE

// -------------------
// Window System API?
// -------------------

use crate::alloc::{borrow::ToOwned, vec, vec::Vec};
use crate::services::sparc::{Sparc, SparcDetails, SparcReq, SparcStatus};

struct ArcWinManager {
    active_windows: Vec<ArcWin>,
    // use arcwin.sparc_status() to determine if the window is foregrounded or backgrounded
}

impl ArcWinManager {
    pub fn new() -> Self {
        Self {
            active_windows: vec![],
        }
    }
    // request a new window be created
    pub fn create_new_window(&mut self) -> &ArcWin {
        let win = ArcWin::new();
        self.active_windows.push(win);
        match self.active_windows.last() {
            Some(_win) => _win,
            // if would create so many windows as to run out of space, just panic and reset
            None => panic!("Could not create window"),
        }
    }
}

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
    fn drag(&mut self, new_coords: &[f32; 2]) {
        // drag the window to the new location
        self.position = new_coords.to_owned();
    }
    fn reset_position(&mut self) {
        self.position = [0.0, 0.0];
    }
    fn sparc_status(&self) -> SparcStatus {
        self.sparc_details.status()
    }
}

impl Sparc for ArcWin {
    // observer pattern
    fn handle_req(sparc_req: SparcReq) {}

    // CREATE A WINDOW AND FOREGROUND IT
    fn start() {}

    // TODO: send close signal to arcwin manager through IPC or directly
    fn close() {}
}

const DEFAULT_Z_INDEX: i32 = 100;
const SCR_HEIGHT: i32 = 1080;
const SCR_WIDTH: i32 = 1920;

// -------------------
// Window System API NEW?
// -------------------

pub struct ArcWM {}

type WindowID = u64;

pub trait WMFunctions {
    fn new() -> Self;
    fn spawn_window(&self) -> WindowID;
    fn del_window(&self, _: WindowID) -> bool;
}

type WindowBuffer = u64;

pub struct Window {
    current_pos: (u64, u64),
    z_index: i64,
    framebuffers: [WindowBuffer; 2],
}

pub trait WindowFunctions {
    fn new() -> Self;
    fn resize(&self, new_size: (u64, u64));
    fn move_to(&self, new_coords: (u64, u64));
    // draw the current framebuffer, then swap to the back one
    fn draw(&self);
    fn swap_framebuffer(&self);
}


#[test_case]
fn test_basics_sparc() {
    let _sparc = ArcWin {
        sparc_details: SparcDetails::new(),
        state: WindowState::FOREGROUND,
        z_index: DEFAULT_Z_INDEX,
        position: [(SCR_WIDTH / 2) as f32, (SCR_HEIGHT / 2) as f32],
        responsive: true,
    };
}
