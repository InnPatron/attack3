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
