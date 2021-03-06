use std::fmt;
use std::rc::Rc;

use super::raw_input::Packet;
use super::config::*;

macro_rules! printHandler {
    ($msg: expr) => (Box::new(|| { println!("{}", $msg); }));
    (joy => $msg: expr) => (Box::new(|_| { println!("{}", $msg); }))
}

macro_rules! handler {
    (UP => $d: expr, $k: expr) => {{
        let tmp = $d.clone();
        Box::new((move || tmp.key_up($k))) as TriggerHandler
    }};

    (UPM => $d: expr, $k: expr) => {{
        let tmp = $d.clone();
        Box::new((move |_| tmp.key_up($k))) as Box<dyn Fn(f32) -> ()>
    }};

    (DOWN => $d: expr, $k: expr) => {{
        let tmp = $d.clone();
        Box::new((move || tmp.key_down($k))) as TriggerHandler
    }};

    (DOWNM => $d: expr, $k: expr) => {{
        let tmp = $d.clone();
        Box::new((move |_| tmp.key_down($k))) as Box<dyn Fn(f32) -> ()>
    }};

    (NOP) => {{
        Box::new(|| ())
    }};

    (NOPM) => {{
        Box::new(|_| ())
    }};
}

pub trait Dispatcher {
    fn from_cfg(cfg: &Config) -> Self;
    fn key_up(&self, k: Key);
    fn key_down(&self, k: Key);
    fn rel_mouse_x(&self, r: i32);
    fn rel_mouse_y(&self, r: i32);
}

pub type TriggerHandler = Box<dyn Fn() -> ()>;

pub struct Manager {
    previous_state: Option<State>,
    button_up: Vec<TriggerHandler>,
    button_down: Vec<TriggerHandler>,

    x_enter_deadzone_negative: TriggerHandler,
    y_enter_deadzone_negative: TriggerHandler,
    x_enter_deadzone_positive: TriggerHandler,
    y_enter_deadzone_positive: TriggerHandler,

    x_exit_deadzone_negative: TriggerHandler,
    y_exit_deadzone_negative: TriggerHandler,
    x_exit_deadzone_positive: TriggerHandler,
    y_exit_deadzone_positive: TriggerHandler,

    axis_tracker: Box<dyn FnMut(f32, f32, f32) -> ()>,

    x_deadzone: f32,
    y_deadzone: f32,
}

impl Manager {

    #[cfg(target_os = "windows")]
    pub fn new(cfg: Config) -> Self {
        #[cfg(target_os = "windows")]
        use super::win_input as winput;

        let dispatcher = Rc::new(winput::WinDispatch::from_cfg(&cfg));

        let mut button_up = Vec::new();
        let mut button_down = Vec::new();
        for k in cfg.buttons.iter().cloned() {
            match k {
                Some(k) => {
                    let d1 = dispatcher.clone();
                    let c1 = move || {
                        // println!("Key up: {:?}", k);
                        d1.key_up(k);
                    };

                    let d2 = dispatcher.clone();
                    let c2 = move || {
                        // println!("Key down: {:?}", k);
                        d2.key_down(k);
                    };

                    button_up.push(Box::new(c1) as TriggerHandler);
                    button_down.push(Box::new(c2) as TriggerHandler);
                }

                None => {
                    button_up.push(handler!(NOP));
                    button_down.push(handler!(NOP));
                }
            }
        }
        match cfg.joystick {
            Some(JoystickConfig::Keys {
                x_axis,
                y_axis
            }) => {
                let x_enter_deadzone_negative =
                    handler!(UP => dispatcher, x_axis.negative);
                let x_exit_deadzone_negative =
                    handler!(DOWN => dispatcher, x_axis.negative);

                let y_enter_deadzone_negative =
                    handler!(UP => dispatcher, y_axis.negative);
                let y_exit_deadzone_negative =
                    handler!(DOWN => dispatcher, y_axis.negative);

                let x_enter_deadzone_positive =
                    handler!(UP => dispatcher, x_axis.positive);
                let x_exit_deadzone_positive =
                    handler!(DOWN => dispatcher, x_axis.positive);

                let y_enter_deadzone_positive =
                    handler!(UP => dispatcher, y_axis.positive);
                let y_exit_deadzone_positive =
                    handler!(DOWN => dispatcher, y_axis.positive);

                Manager {
                    previous_state: None,

                    button_up,
                    button_down,

                    x_enter_deadzone_negative,
                    y_enter_deadzone_negative,

                    x_enter_deadzone_positive,
                    y_enter_deadzone_positive,

                    x_exit_deadzone_negative,
                    y_exit_deadzone_negative,

                    x_exit_deadzone_positive,
                    y_exit_deadzone_positive,

                    // NOP
                    axis_tracker: Box::new(|_, _, _| ()),

                    x_deadzone: x_axis.deadzone,
                    y_deadzone: y_axis.deadzone,
                }
            }

            Some(JoystickConfig::Mouse {
                x_axis,
                y_axis,
            }) => {

                Manager {
                    previous_state: None,

                    button_up,
                    button_down,

                    x_enter_deadzone_negative: handler!(NOP),
                    y_enter_deadzone_negative: handler!(NOP),

                    x_enter_deadzone_positive: handler!(NOP),
                    y_enter_deadzone_positive: handler!(NOP),

                    x_exit_deadzone_negative: handler!(NOP),
                    y_exit_deadzone_negative: handler!(NOP),

                    x_exit_deadzone_positive: handler!(NOP),
                    y_exit_deadzone_positive: handler!(NOP),

                    axis_tracker: {
                        let mut x_handler = Manager::mouse_mode_handler(
                            dispatcher.clone(),
                            Axis::X,
                            x_axis.clone(),
                        );
                        let mut y_handler = Manager::mouse_mode_handler(
                            dispatcher.clone(),
                            Axis::Y,
                            y_axis.clone(),
                        );
                        Box::new(move |x, y, _| {
                            x_handler(x);
                            y_handler(y);
                        })
                    },

                    x_deadzone: x_axis.deadzone,
                    y_deadzone: y_axis.deadzone,
                }
            }

            None => Manager {
                previous_state: None,

                button_up,
                button_down,

                x_enter_deadzone_negative: handler!(NOP),
                y_enter_deadzone_negative: handler!(NOP),

                x_enter_deadzone_positive: handler!(NOP),
                y_enter_deadzone_positive: handler!(NOP),

                x_exit_deadzone_negative: handler!(NOP),
                y_exit_deadzone_negative: handler!(NOP),

                x_exit_deadzone_positive: handler!(NOP),
                y_exit_deadzone_positive: handler!(NOP),

                axis_tracker: Box::new(|_, _, _| ()),

                x_deadzone: 2.0,
                y_deadzone: 2.0,
            }
        }
    }

    fn mouse_mode_handler<T: 'static + Dispatcher>(
        dispatcher: Rc<T>,
        axis: Axis,
        config: AxisMouseConfig,
    ) -> Box<dyn FnMut(f32) -> ()> {
        // NOTE: input per axis comes from a State
        //   Thus, normalized [-1.0, 1.0]
        //   Map directly to an 'inch'
        match config.dots_per_pixel {
            MouseMode::Constant(dots_per_pixel) => {
                let mut dots_moved_acc = 0.0;
                Box::new(move |f| {
                    if f.abs() < config.deadzone {
                        return;
                    }

                    let inches_moved = f;
                    let dots_moved = inches_moved * config.dpi;
                    dots_moved_acc += dots_moved;

                    // Apply dots-per-pixel-function:
                    //   g(f) = c
                    let pixels_moved = (dots_moved_acc / dots_per_pixel) as i32;
                    dots_moved_acc = dots_moved_acc % dots_per_pixel;

                    match axis {
                        Axis::X => dispatcher.rel_mouse_x(pixels_moved),
                        Axis::Y => dispatcher.rel_mouse_y(pixels_moved),
                        Axis::Z => panic!("Cannot have mouse Z axis movement"),
                    }
                })
            },

            MouseMode::Linear { m, bias } => {
                let mut dots_moved_acc = 0.0;
                Box::new(move |f| {
                    if f.abs() < config.deadzone {
                        return;
                    }

                    let inches_moved = f;
                    let dots_moved = inches_moved * config.dpi;
                    dots_moved_acc += dots_moved;

                    // Apply dots-per-pixel-function:
                    //   g(f) = (1 - |f|) * m + (bias * sign(m))
                    let bias = bias * m.signum();
                    let dots_per_pixel = (1.0 - f.abs()) * m + bias;

                    let pixels_moved =  (dots_moved_acc / dots_per_pixel) as i32;
                    dots_moved_acc = dots_moved_acc % dots_per_pixel;

                    match axis {
                        Axis::X => dispatcher.rel_mouse_x(pixels_moved),
                        Axis::Y => dispatcher.rel_mouse_y(pixels_moved),
                        Axis::Z => panic!("Cannot have mouse Z axis movement"),
                    }
                })
            },

            MouseMode::Logistic { target, min, a, b, c, d, h } => {
                let mut dots_moved_acc = 0.0;
                Box::new(move |f| {
                    if f.abs() < config.deadzone {
                        return;
                    }

                    let inches_moved = f;
                    let dots_moved = inches_moved * config.dpi;
                    dots_moved_acc += dots_moved;

                    // Apply dots-per-pixel-function g:
                    //   coef(f) = [1 / (c + (b * e)^(|f| + d))] * a + h
                    //   g(f) = minimum(|clamp((1 - coef(f)), 0, 1) * target|, |min|) * sign(target)
                    let coef = {
                        let exponent = f.abs() + d;
                        let denom = c + b.powf(-exponent) * (-exponent).exp();
                        1.0 / denom * a + h
                    };
                    let coef = coef.clamp(0.0, 1.0);
                    let mut dots_per_pixel = (1.0 - coef) * target;

                    if dots_per_pixel.abs() < min {
                        dots_per_pixel = min * dots_per_pixel.signum();
                    }

                    let pixels_moved = if dots_per_pixel != 0.0 {
                        let pixels_moved = (dots_moved_acc / dots_per_pixel) as i32;
                        dots_moved_acc = dots_moved_acc % dots_per_pixel;
                        pixels_moved
                    } else {
                        0
                    };

                    match axis {
                        Axis::X => dispatcher.rel_mouse_x(pixels_moved),
                        Axis::Y => dispatcher.rel_mouse_y(pixels_moved),
                        Axis::Z => panic!("Cannot have mouse Z axis movement"),
                    }
                })
            },
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
            x_enter_deadzone_negative: printHandler!("x-axis enter deadzone -"),
            y_enter_deadzone_negative: printHandler!("y-axis enter deadzone -"),

            x_enter_deadzone_positive: printHandler!("x-axis enter deadzone +"),
            y_enter_deadzone_positive: printHandler!("y-axis enter deadzone +"),

            x_exit_deadzone_negative: printHandler!("x-axis exit deadzone -"),
            y_exit_deadzone_negative: printHandler!("y-axis exit deadzone -"),

            x_exit_deadzone_positive: printHandler!("x-axis exit deadzone +"),
            y_exit_deadzone_positive: printHandler!("y-axis exit deadzone +"),

            // NOP
            axis_tracker: Box::new(|_, _, _| ()),

            x_deadzone: 0.5,
            y_deadzone: 0.5,
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

                (self.axis_tracker)(ns.x_axis, ns.y_axis, 0.0);

                if ps_x > self.x_deadzone && ns_x <= self.x_deadzone {
                    if ns.x_axis >= 0.0 {
                        (self.x_enter_deadzone_positive)();
                    } else {
                        (self.x_enter_deadzone_negative)();
                    }
                } else if ps_x <= self.x_deadzone && ns_x > self.x_deadzone {
                    if ns.x_axis >= 0.0 {
                        (self.x_exit_deadzone_positive)();
                    } else {
                        (self.x_exit_deadzone_negative)();
                    }
                }

                if ps_y > self.y_deadzone && ns_y <= self.y_deadzone {
                    if ns.y_axis >= 0.0 {
                        (self.y_enter_deadzone_positive)();
                    } else {
                        (self.y_enter_deadzone_negative)();
                    }
                } else if ps_y <= self.y_deadzone && ns_y > self.y_deadzone {
                    if ns.y_axis >= 0.0 {
                        (self.y_exit_deadzone_positive)();
                    } else {
                        (self.y_exit_deadzone_negative)();
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
    /// Normalized in range [-1.0, 1.0] [left, right]
    x_axis: f32,
    /// Normalized in range [-1.0, 1.0] [back, forward]
    y_axis: f32,
    /// Normalized in range [-1.0, 1.0] [down, up]
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
