use std::fmt;

use super::Packet;

pub const BUTTON_LEN: usize = 11;

macro_rules! printHandler {
    ($msg: expr) => (Box::new(|| { println!("{}", $msg); }));
    (joy => $msg: expr) => (Box::new(|_| { println!("{}", $msg); }))
}

pub trait Dispatcher {
    fn from_cfg(cfg: &Config) -> Self;
    fn key_up(&self, k: Key);
    fn key_down(&self, k: Key);
}

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

pub struct Manager {
    previous_state: Option<State>,
    button_up: Vec<Box<dyn Fn() -> ()>>,
    button_down: Vec<Box<dyn Fn() -> ()>>,

    x_enter_dead_zone_negative: Box<dyn Fn() -> ()>,
    y_enter_dead_zone_negative: Box<dyn Fn() -> ()>,
    x_enter_dead_zone_positive: Box<dyn Fn() -> ()>,
    y_enter_dead_zone_positive: Box<dyn Fn() -> ()>,

    x_exit_dead_zone_negative: Box<dyn Fn() -> ()>,
    y_exit_dead_zone_negative: Box<dyn Fn() -> ()>,
    x_exit_dead_zone_positive: Box<dyn Fn() -> ()>,
    y_exit_dead_zone_positive: Box<dyn Fn() -> ()>,

    x_dead_zone: f32,
    y_dead_zone: f32,
}

impl Manager {

    #[cfg(target_os = "windows")]
    pub fn new(cfg: Config) -> Self {
        #[cfg(target_os = "windows")]
        use super::win_input as winput;
        use std::rc::Rc;

        let dispatcher = Rc::new(winput::WinDispatch::from_cfg(&cfg));

        let mut button_up = Vec::new();
        let mut button_down = Vec::new();
        for k in cfg.buttons.iter().cloned() {
            let d1 = dispatcher.clone();
            let c1 = move || {
                println!("Key up: {:?}", k);
                d1.key_up(k);
            };

            let d2 = dispatcher.clone();
            let c2 = move || {
                println!("Key down: {:?}", k);
                d2.key_down(k);
            };

            button_up.push(Box::new(c1) as Box<dyn Fn() -> ()>);
            button_down.push(Box::new(c2) as Box<dyn Fn() -> ()>);
        }
        Manager {
            previous_state: None,

            button_up,
            button_down,

            x_enter_dead_zone_negative: printHandler!("x-axis enter deadzone -"),
            y_enter_dead_zone_negative: printHandler!("y-axis enter deadzone -"),

            x_enter_dead_zone_positive: printHandler!("x-axis enter deadzone +"),
            y_enter_dead_zone_positive: printHandler!("y-axis enter deadzone +"),

            x_exit_dead_zone_negative: printHandler!("x-axis exit deadzone -"),
            y_exit_dead_zone_negative: printHandler!("y-axis exit deadzone -"),

            x_exit_dead_zone_positive: printHandler!("x-axis exit deadzone +"),
            y_exit_dead_zone_positive: printHandler!("y-axis exit deadzone +"),


            x_dead_zone: cfg.x_dead_zone,
            y_dead_zone: cfg.y_dead_zone,
        }
    }

    #[allow(dead_code)]
    pub fn dbg() -> Self {
        Manager {
            previous_state: None,
            button_up: vec![
                printHandler!("up button1"),
                printHandler!("up button2"),
                printHandler!("up button3"),
                printHandler!("up button4"),
                printHandler!("up button5"),
                printHandler!("up button6"),
                printHandler!("up button7"),
                printHandler!("up button8"),
                printHandler!("up button9"),
                printHandler!("up button10"),
                printHandler!("up button11"),
            ],
            button_down: vec![
                printHandler!("down button1"),
                printHandler!("down button2"),
                printHandler!("down button3"),
                printHandler!("down button4"),
                printHandler!("down button5"),
                printHandler!("down button6"),
                printHandler!("down button7"),
                printHandler!("down button8"),
                printHandler!("down button9"),
                printHandler!("down button10"),
                printHandler!("down button11"),
            ],
            x_enter_dead_zone_negative: printHandler!("x-axis enter deadzone -"),
            y_enter_dead_zone_negative: printHandler!("y-axis enter deadzone -"),

            x_enter_dead_zone_positive: printHandler!("x-axis enter deadzone +"),
            y_enter_dead_zone_positive: printHandler!("y-axis enter deadzone +"),

            x_exit_dead_zone_negative: printHandler!("x-axis exit deadzone -"),
            y_exit_dead_zone_negative: printHandler!("y-axis exit deadzone -"),

            x_exit_dead_zone_positive: printHandler!("x-axis exit deadzone +"),
            y_exit_dead_zone_positive: printHandler!("y-axis exit deadzone +"),


            x_dead_zone: 0.5,
            y_dead_zone: 0.5,
        }
    }

    pub fn step(&mut self, ns: State) {
        match self.previous_state.take() {
            Some(ps) => {
                for i in 0..BUTTON_LEN {
                    let pb = ps.buttons[i];
                    let nb = ns.buttons[i];

                    // println!("{}: {} -> {}", i + 1, pb, nb);

                    if !pb && nb {
                        self.button_down[i]();
                    } else if pb && !nb {
                        self.button_up[i]();
                    }
                }

                // TODO: z-axis support
                let ps_x = ps.x_axis.abs();
                let ps_y = ps.y_axis.abs();
                // let ps_z = ps.z_axis.abs();

                let ns_x = ns.x_axis.abs();
                let ns_y = ns.y_axis.abs();
                // let ns_z = ns.z_axis.abs();

                if ps_x > self.x_dead_zone && ns_x <= self.x_dead_zone {
                    if ns.x_axis >= 0.0 {
                        (self.x_enter_dead_zone_positive)();
                    } else {
                        (self.x_enter_dead_zone_negative)();
                    }
                } else if ps_x <= self.x_dead_zone && ns_x > self.x_dead_zone {
                    if ns.x_axis >= 0.0 {
                        (self.x_exit_dead_zone_positive)();
                    } else {
                        (self.x_exit_dead_zone_negative)();
                    }
                }

                if ps_y > self.y_dead_zone && ns_y <= self.y_dead_zone {
                    if ns.y_axis >= 0.0 {
                        (self.y_enter_dead_zone_positive)();
                    } else {
                        (self.y_enter_dead_zone_negative)();
                    }
                } else if ps_y <= self.y_dead_zone && ns_y > self.y_dead_zone {
                    if ns.y_axis >= 0.0 {
                        (self.y_exit_dead_zone_positive)();
                    } else {
                        (self.y_exit_dead_zone_negative)();
                    }
                }

                self.previous_state = Some(ns);
            }

            None => {
                self.previous_state = Some(ns);

            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct State {
    buttons: [bool; 11],
    x_axis: f32,
    y_axis: f32,
    z_axis: f32,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, b) in self.buttons.iter().enumerate() {
            write!(f, "button {}: {}\n", i + 1, b)?;
        }

        write!(f, "x-axis: {}\n", self.x_axis)?;
        write!(f, "y-axis: {}\n", self.y_axis)?;
        write!(f, "z-axis: {}\n", self.z_axis)?;


        Ok(())
    }
}

impl State {
    /// Normalize axis values between [-1, 1]
    /// x-axis:
    ///   * +1 => right
    ///   * -1 => left
    /// y-axis:
    ///   * +1 => forward
    ///   * -1 => backward
    /// z-axis:
    ///   * +1 => up
    ///   * -1 => down
    pub fn from_packet(zero: [u8; 2], packet: Packet) -> Self {
        let x_zero = zero[0];
        let y_zero = zero[1];

        let px = packet.x_axis as f32;
        let py = packet.y_axis as f32;
        let pz = packet.z_axis as f32;

        let sx = 2.0 * (px - x_zero as f32) / 255.0;
        let sy = -2.0 * (py - y_zero as f32) / 255.0;
        let sz = -2.0 * (pz - 128.0) / 255.0;

        State {
            buttons: packet.buttons,
            x_axis: sx,
            y_axis: sy,
            z_axis: sz,
        }
    }
}
