fn main() {
    windows::build!(
        // windows::win32::keyboard_and_mouse_input,
        windows::win32::keyboard_and_mouse_input::{
            INPUT,
            SendInput,
            INPUT,
            MOUSEINPUT,
            KEYBDINPUT,
            HARDWAREINPUT,
        },
        windows::win32::windows_and_messaging::{
            GetMessageExtraInfo,
        },
    );
}
