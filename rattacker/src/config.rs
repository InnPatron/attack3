pub const BUTTON_LEN: usize = 11;

// TODO: z-axis
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Config {
    pub mode: Mode,
    pub buttons: [Key; BUTTON_LEN],
    pub joystick: JoystickConfig,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Normal,
    DirectX,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JoystickConfig {
    Keys {
        x_axis: AxisKeyConfig,
        y_axis: AxisKeyConfig,
    },

    Mouse {
        x_axis: AxisMouseConfig,
        y_axis: AxisMouseConfig,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AxisMouseConfig {
    pub mouse_mode: MouseMode,
    pub deadzone: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseMode {
    Constant(i32),
    Linear {
        min: i32,
        max: i32,
        m: i32,
        b: i32,
    },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AxisKeyConfig {
    pub positive: Key,
    pub negative: Key,
    pub deadzone: f32,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Alt,
    Shift,
    Ctrl,
    Enter,
    Escape,

    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,

    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
}
