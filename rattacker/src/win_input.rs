use std::mem;
use std::collections::HashMap;

use super::dispatch::Dispatcher;
use super::config::{ Config, Key, Mode, Axis};

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

pub struct WinDispatch {
    mode: Mode,
    key_down: HashMap<Key, Input>,
    key_up: HashMap<Key, Input>,
}

impl WinDispatch {
    fn cache_key(&mut self, k: &Key, mode: Mode) {
        if self.key_down.contains_key(k) {
            return;
        }

        match mode {
            Mode::DirectX => {
                self.key_down.insert(k.clone(),
                Input::new_directx_key(k.clone(), false));
                self.key_up.insert(k.clone(),
                Input::new_directx_key(k.clone(), true));
            }

            Mode::Normal => {
                self.key_down.insert(k.clone(),
                Input::new_ascii_key(k.clone(), false));
                self.key_up.insert(k.clone(),
                Input::new_ascii_key(k.clone(), true));

            }
        }
    }
}

impl Dispatcher for WinDispatch {
    fn from_cfg(cfg: &Config) -> Self {
        use super::config::*;

        let mut disp = WinDispatch {
            mode: cfg.mode.clone(),
            key_down: HashMap::new(),
            key_up: HashMap::new(),
        };

        for k in cfg.buttons.iter() {
            disp.cache_key(k, cfg.mode);
        }

        match cfg.joystick {
            JoystickConfig::Keys {
                x_axis,
                y_axis
            } => {
                let joy_keys = [
                    x_axis.positive,
                    x_axis.negative,
                    y_axis.positive,
                    y_axis.negative,
                ];
                // dbg!(joy_keys);

                for k in joy_keys.iter() {
                    disp.cache_key(k, cfg.mode);
                }

                disp
            }

            JoystickConfig::Mouse {
                x_axis,
                y_axis,
            } => {
                // Do no pre-caching for now
                disp
            }
        }
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

    fn rel_mouse_x(&self, r: i32) {
        // println!("Attempting to send rel_x: {}", r);
        match self.mode {
            Mode::DirectX | Mode::Normal => {
                unsafe {
                    // TODO: check if we need to extend the lifetime of input
                    let input = Input::new_mouse_rel(r, 0);
                    let input = &input as *const _;
                    SendInput(1, mem::transmute(input),
                    mem::size_of::<Input>() as i32);
                }
            }
        }
    }

    fn rel_mouse_y(&self, r: i32) {
        // println!("Attempting to send rel_y: {}", r);
        match self.mode {
            Mode::DirectX | Mode::Normal => {
                unsafe {
                    // TODO: check if we need to extend the lifetime of input
                    let input = Input::new_mouse_rel(0, r);
                    let input = &input as *const _;
                    SendInput(1, mem::transmute(input),
                    mem::size_of::<Input>() as i32);
                }
            }
        }
    }
}

fn ascii_virtual_key(k: Key) -> u16 {
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
        Key::Escape => 0x1B,

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

// DirectX has its own keyboard scancodes
fn directx_virtual_key(k: Key) -> u16 {
    match k {
        Key::A => 0x1E,
        Key::B => 0x30,
        Key::C => 0x2E,
        Key::D => 0x20,
        Key::E => 0x12,
        Key::F => 0x21,
        Key::G => 0x22,
        Key::H => 0x23,
        Key::I => 0x17,
        Key::J => 0x24,
        Key::K => 0x25,
        Key::L => 0x26,
        Key::M => 0x32,
        Key::N => 0x31,
        Key::O => 0x18,
        Key::P => 0x19,
        Key::Q => 0x10,
        Key::R => 0x13,
        Key::S => 0x1F,
        Key::T => 0x14,
        Key::U => 0x16,
        Key::V => 0x2F,
        Key::W => 0x11,
        Key::X => 0x2D,
        Key::Y => 0x15,
        Key::Z => 0x2C,

        Key::K0 => 0x0B,
        Key::K1 => 0x02,
        Key::K2 => 0x03,
        Key::K3 => 0x04,
        Key::K4 => 0x05,
        Key::K5 => 0x06,
        Key::K6 => 0x07,
        Key::K7 => 0x08,
        Key::K8 => 0x09,
        Key::K9 => 0x0A,

        Key::Enter => 0x1C,
        Key::Shift => 0x36,
        Key::Ctrl => 0x1d,
        Key::Alt => 0x38,

        Key::LeftArrow => 0xCB,
        Key::UpArrow => 0xC8,
        Key::RightArrow => 0xCD,
        Key::DownArrow => 0xD0,
        Key::Escape => 0x01,

        Key::F1 => 0x3B,
        Key::F2 => 0x3C,
        Key::F3 => 0x3D,
        Key::F4 => 0x3E,
        Key::F5 => 0x3F,
        Key::F6 => 0x40,
        Key::F7 => 0x41,
        Key::F8 => 0x42,
        Key::F9 => 0x43,

        #[allow(unreachable_patterns)]
        k => todo!("Windows virtual key: {:?}", k),
    }
}

const MOUSEEVENTF_MOVE: u32 = 0x0001;
const TAG_MOUSE: u32 = 0;
const TAG_KEY: u32 = 1;
const KEY_UP: u32 = 0x0002;

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
    fn new_directx_key(k: Key, up: bool) -> Self {
        Input {
            tag: TAG_KEY,
            union: InputUnion {
                ki: mem::ManuallyDrop::new(KEYBDINPUT {
                    w_vk: 0,
                    w_scan: directx_virtual_key(k),
                    dw_flags: if up { KEY_UP } else { 0x0 } | 0x0004 | 0x0008,
                    time: 0,
                    dw_extra_info: unsafe { GetMessageExtraInfo() }.0 as usize,
                }),
            },
        }
    }

    fn new_ascii_key(k: Key, up: bool) -> Self {
        Input {
            tag: TAG_KEY,
            union: InputUnion {
                ki: mem::ManuallyDrop::new(KEYBDINPUT {
                    w_vk: ascii_virtual_key(k),
                    w_scan: 0,
                    dw_flags: if up { KEY_UP } else { 0x0 },
                    time: 0,
                    dw_extra_info: unsafe { GetMessageExtraInfo() }.0 as usize,
                }),
            },
        }
    }

    fn new_mouse_rel(x_amount: i32, y_amount: i32) -> Self {
        Input {
            tag: TAG_MOUSE,
            union: InputUnion {
                mi: mem::ManuallyDrop::new(MOUSEINPUT {
                    dx: x_amount,
                    dy: y_amount,
                    mouse_data: 0x0,
                    dw_flags: MOUSEEVENTF_MOVE,
                    time: 0,
                    dw_extra_info: unsafe { GetMessageExtraInfo() }.0 as usize,
                })
            }
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
