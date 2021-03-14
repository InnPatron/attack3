use serde::{Serialize, Deserialize};

pub const BUTTON_LEN: usize = 11;

// TODO: z-axis
// NOTE: polling delay should NOT be configurable
// hidapi accumulates packets and returns  1 packet per read()
//   Delaying a read() causes packets to accumulate, leading to laggy input
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub mode: Mode,
    pub buttons: [Option<Key>; BUTTON_LEN],
    pub joystick: JoystickConfig,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mode {
    Normal,
    DirectX,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
/// Implements mouse-style input
/// Maps the normalized axis value ([-1.0, 1.0]) directly to inches moved
pub struct AxisMouseConfig {
    /// Defines the dots-per-pixel function
    pub dots_per_pixel: MouseMode,
    pub dpi: f32,
    pub deadzone: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum MouseMode {

    /// g(f) = c
    Constant(f32),

    /// g(f) = (1 - |f|) * m + (bias * sign(m))
    Linear {
        m: f32,
        bias: f32,
    },

    /// coef(f) = [1 / (c + (b * e)^(|f| + d))] * a + h
    /// g(f) = coef(f) * target
    Logistic {
        target: f32,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        h: f32,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisKeyConfig {
    pub positive: Key,
    pub negative: Key,
    pub deadzone: f32,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    LMB,
    RMB,
}

impl Key {
    pub fn is_mouse(&self) -> bool {
        *self == Key::LMB || *self == Key::RMB
    }
}
