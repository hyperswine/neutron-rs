// WINDOW SERVER LIBRARY THAT LINKS WITH NEUTRON CORE

// -------------------
// Window System API
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
