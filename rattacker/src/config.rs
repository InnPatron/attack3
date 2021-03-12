pub const BUTTON_LEN: usize = 11;

// TODO: z-axis
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Config {
    pub buttons: [Key; BUTTON_LEN],
    pub x_axis_positive: Key,
    pub x_axis_negative: Key,

    pub y_axis_positive: Key,
    pub y_axis_negative: Key,

    pub x_dead_zone: f32,
    pub y_dead_zone: f32,
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
