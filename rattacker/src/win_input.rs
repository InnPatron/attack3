use std::mem;
use std::collections::HashMap;

use super::dispatch::{ Config, Dispatcher, Key };

use bindings::windows::win32::keyboard_and_mouse_input::{
    SendInput,
    // INPUT,
    MOUSEINPUT,
    KEYBDINPUT,
    HARDWAREINPUT,
};

use bindings::windows::win32::windows_and_messaging::{
    GetMessageExtraInfo,
};

const TAG_KEY: u32 = 1;
const KEY_UP: u32 = 0x0002;

pub struct WinDispatch {
    key_down: HashMap<Key, Input>,
    key_up: HashMap<Key, Input>,
}

impl Dispatcher for WinDispatch {
    fn from_cfg(cfg: &Config) -> Self {
        let mut disp = WinDispatch {
            key_down: HashMap::new(),
            key_up: HashMap::new(),
        };

        for k in cfg.buttons.iter() {
            if disp.key_down.contains_key(k) {
                continue;
            }

            disp.key_down.insert(k.clone(), Input::new(k.clone(), false));
            disp.key_up.insert(k.clone(), Input::new(k.clone(), true));
        }

        disp
    }

    fn key_up(&self, k: Key) {
        let input = self.key_up.get(&k).unwrap() as *const _ as *mut Input;

        // Will never modify self.key_up after from_cfg()
        unsafe {
            SendInput(1,
                      mem::transmute(input),     // TODO: remove when windows bindings can handle INPUT
                      mem::size_of::<Input>() as i32);
        }
    }

    fn key_down(&self, k: Key) {
        let input = self.key_down.get(&k).unwrap() as *const _;

        // Will never modify self.key_up after from_cfg()
        unsafe {
            SendInput(1,
                      mem::transmute(input),     // TODO: remove when windows bindings can handle INPUT
                      mem::size_of::<Input>() as i32);
        }
    }
}

fn to_virtual_key(k: Key) -> u16 {
    match k {
        Key::A => 0x41,
        Key::B => 0x42,
        Key::C => 0x43,
        Key::D => 0x44,
        Key::E => 0x45,
        Key::F => 0x46,
        Key::G => 0x47,
        Key::H => 0x48,
        Key::I => 0x49,
        Key::J => 0x4A,
        Key::K => 0x4B,
        Key::L => 0x4C,
        Key::M => 0x4D,
        Key::N => 0x4E,
        Key::O => 0x4F,
        Key::P => 0x50,
        Key::Q => 0x51,
        Key::R => 0x52,
        Key::S => 0x53,
        Key::T => 0x54,
        Key::U => 0x55,
        Key::V => 0x56,
        Key::W => 0x57,
        Key::X => 0x58,
        Key::Y => 0x59,
        Key::Z => 0x5A,

        Key::K0 => 0x30,
        Key::K1 => 0x31,
        Key::K2 => 0x32,
        Key::K3 => 0x33,
        Key::K4 => 0x34,
        Key::K5 => 0x35,
        Key::K6 => 0x36,
        Key::K7 => 0x37,
        Key::K8 => 0x38,
        Key::K9 => 0x39,

        Key::Enter => 0x0D,
        Key::Shift => 0x10,
        Key::Ctrl => 0x11,
        Key::Alt => 0x12,

        Key::LeftArrow => 0x25,
        Key::UpArrow => 0x26,
        Key::RightArrow => 0x27,
        Key::DownArrow => 0x28,

        Key::F1 => 0x70,
        Key::F2 => 0x71,
        Key::F3 => 0x72,
        Key::F4 => 0x73,
        Key::F5 => 0x74,
        Key::F6 => 0x75,
        Key::F7 => 0x76,
        Key::F8 => 0x77,
        Key::F9 => 0x78,

        #[allow(unreachable_patterns)]
        k => todo!("Windows virtual key: {:?}", k),
    }
}

/// TODO: windows bindings does not support the INPUT type
///typedef struct tagINPUT {
///  DWORD type;
///  union {
///    MOUSEINPUT    mi;
///    KEYBDINPUT    ki;
///    HARDWAREINPUT hi;
///  } DUMMYUNIONNAME;
///} INPUT, *PINPUT, *LPINPUT;
#[repr(C)]
struct Input {
    tag: u32,
    union: InputUnion,
}

impl Input {
    fn new(k: Key, up: bool) -> Self {
        Input {
            tag: TAG_KEY,
            union: InputUnion {
                ki: mem::ManuallyDrop::new(KEYBDINPUT {
                    w_vk: to_virtual_key(k),
                    w_scan: 0,
                    dw_flags: if up { KEY_UP } else { 0x0 },
                    time: 0,
                    dw_extra_info: unsafe { GetMessageExtraInfo() }.0 as usize,
                }),
            },
        }
    }
}

/// Never need to manually drop b/c program will be halted
#[repr(C)]
union InputUnion {
    mi: mem::ManuallyDrop<MOUSEINPUT>,
    ki: mem::ManuallyDrop<KEYBDINPUT>,
    hi: mem::ManuallyDrop<HARDWAREINPUT>,
}
