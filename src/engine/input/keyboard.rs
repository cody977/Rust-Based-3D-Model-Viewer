//get user keyboard input from HashMap.
//(No RL bindings)

use std::collections::HashMap;

#[link(name = "user32")]
unsafe extern "system" {
    fn GetAsyncKeyState(key: i32) -> i16;
}

// Virtual key codes
pub const VK_BACKSPACE: i32 = 0x08;
pub const VK_TAB: i32 = 0x09;
pub const VK_ENTER: i32 = 0x0D;
pub const VK_SHIFT: i32 = 0x10;
pub const VK_CTRL: i32 = 0x11;
pub const VK_ALT: i32 = 0x12;
pub const VK_ESCAPE: i32 = 0x1B;
pub const VK_SPACE: i32 = 0x20;

pub const VK_LEFT: i32 = 0x25;
pub const VK_UP: i32 = 0x26;
pub const VK_RIGHT: i32 = 0x27;
pub const VK_DOWN: i32 = 0x28;

// Numbers 0-9
pub const VK_0: i32 = 0x30;
pub const VK_1: i32 = 0x31;
pub const VK_2: i32 = 0x32;
pub const VK_3: i32 = 0x33;
pub const VK_4: i32 = 0x34;
pub const VK_5: i32 = 0x35;
pub const VK_6: i32 = 0x36;
pub const VK_7: i32 = 0x37;
pub const VK_8: i32 = 0x38;
pub const VK_9: i32 = 0x39;

// Letters A-Z
pub const VK_A: i32 = 0x41;
pub const VK_B: i32 = 0x42;
pub const VK_C: i32 = 0x43;
pub const VK_D: i32 = 0x44;
pub const VK_E: i32 = 0x45;
pub const VK_F: i32 = 0x46;
pub const VK_G: i32 = 0x47;
pub const VK_H: i32 = 0x48;
pub const VK_I: i32 = 0x49;
pub const VK_J: i32 = 0x4A;
pub const VK_K: i32 = 0x4B;
pub const VK_L: i32 = 0x4C;
pub const VK_M: i32 = 0x4D;
pub const VK_N: i32 = 0x4E;
pub const VK_O: i32 = 0x4F;
pub const VK_P: i32 = 0x50;
pub const VK_Q: i32 = 0x51;
pub const VK_R: i32 = 0x52;
pub const VK_S: i32 = 0x53;
pub const VK_T: i32 = 0x54;
pub const VK_U: i32 = 0x55;
pub const VK_V: i32 = 0x56;
pub const VK_W: i32 = 0x57;
pub const VK_X: i32 = 0x58;
pub const VK_Y: i32 = 0x59;
pub const VK_Z: i32 = 0x5A;

// F keys
pub const VK_F1: i32 = 0x70;
pub const VK_F2: i32 = 0x71;
pub const VK_F3: i32 = 0x72;
pub const VK_F4: i32 = 0x73;
pub const VK_F5: i32 = 0x74;
pub const VK_F6: i32 = 0x75;
pub const VK_F7: i32 = 0x76;
pub const VK_F8: i32 = 0x77;
pub const VK_F9: i32 = 0x78;
pub const VK_F10: i32 = 0x79;
pub const VK_F11: i32 = 0x7A;
pub const VK_F12: i32 = 0x7B;

// All keys to poll
const TRACKED_KEYS: &[i32] = &[
    VK_BACKSPACE, VK_TAB, VK_ENTER, VK_SHIFT, VK_CTRL, VK_ALT, VK_ESCAPE, VK_SPACE,
    VK_LEFT, VK_UP, VK_RIGHT, VK_DOWN,
    VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9,
    VK_A, VK_B, VK_C, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_J, VK_K, VK_L, VK_M,
    VK_N, VK_O, VK_P, VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V, VK_W, VK_X, VK_Y, VK_Z,
    VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12,
];

pub struct InputState {
    current: HashMap<i32, bool>, //this frame (pressed)
    previous: HashMap<i32, bool>, //last frame (released)
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            current: HashMap::new(),
            previous: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current); // move current frame to previous
        self.current.clear();                                   // wipe current

        //Poll events (virtual key codes)
        for key in [0x41..=0x5A, 0x30..=0x39].into_iter().flatten() // A-Z, 0-9
            .chain([0x20, 0x1B, 0x0D, 0x08].into_iter()) // space, esc, enter, backspace
        {
            self.current.insert(key, raw_key_down(key));
        }
    }

    pub fn is_down(&self, key: i32) -> bool {
        *self.current.get(&key).unwrap_or(&false)
    }

    pub fn is_pressed(&self, key: i32) -> bool {
        self.is_down(key) && !*self.previous.get(&key).unwrap_or(&false)
    }

    pub fn just_released(&self, key: i32) -> bool {
        !self.is_down(key) && *self.previous.get(&key).unwrap_or(&false)
    }
}

fn raw_key_down(key: i32) -> bool {
    unsafe { GetAsyncKeyState(key) & 0x8000u16  as i16 != 0 }
}
