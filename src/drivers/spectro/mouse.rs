pub enum MouseButtons {
    LEFT_CLICK, RIGHT_CLICK, MIDDLE_CLICK, L1, L2, L3, LMID, R1, R2, R3, R4
}

#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::vec;


// when a mouse button is pressed, will inform the interrupt multiplexer
// happens as quickly as clk or something or to match human reflexes somehow

// It should take in a bunch of values from the firmware/multiplexer cpu
fn handle_mouse_interrupt(clicks: Vec<u8>) -> Vec<MouseButtons> {
    // for GUI, tell the foreground GUI process which buttons were pressed
    vec![MouseButtons::LEFT_CLICK]
}
