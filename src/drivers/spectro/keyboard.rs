// communicate with built in KB line at UART2 or some kind of 

// IMPORTS
use alloc::vec::Vec;
use alloc::vec;

const DEFAULT_KB_LINE: u8 = 2;

pub enum Key {
    // all the keys on the spectro kb
    // and custom keys like left and right stuff
    L_SHIFT, META, L_ALT, SPACE, FN,
    R_MACRO_1,
    R_MACRO_2,
    R_MACRO_3,
    UP, LEFT, DOWN, RIGHT,
    // KB firmware keeps the state of caps lock
    // hm maybe can also detect if stuff like shift and fn are held, or kernel software is fine too for higher level control
    TILDE, TAB, CAPS_ON, CAPS_OFF,
    _1,_2,_3,_4,_5,_6,_7,_8,_9,_0,MINUS,EQUALS,BACKSPACE,DEL,// printSCR,
    F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
    LEFT_ARROW,RIGHT_ARROW,FORWARD_SLASH,BACK_SLASH,BRACKET_LEFT,BRACKET_RIGHT,
    ENTER
}

// basically read() when the keyboard causes an interrupt at the interrupt multiplexer
fn handle_kb_interrupt(keys: Vec<u8>) -> Vec<Key> {
    // switch statements or dictionary to map u8 to key
    let result: Vec<Key> = Vec::new();

    let mapping = {
        // 0 : L_SHIFT // something like this
    };

    result
}
