extern crate hidapi;
#[cfg(target_os = "windows")]
extern crate bindings;

use std::{thread, time};
use std::error::Error;

use hidapi::{HidApi};

mod raw_input;
mod config;

#[macro_use]
mod dispatch;

#[cfg(target_os = "windows")]
mod win_input;

use raw_input::*;
use dispatch::*;
use config::*;

const PACKET_LENGTH: usize = 5;
const VID: u16 = 0x046d;
const PID: u16 = 0xc214;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config {
        mode: Mode::Normal,
        buttons: [
            // b1
            Key::LMB,
            // b2
            Key::K2,
            // b3
            Key::RMB,
            // b4
            Key::K4,
            // b5
            Key::K5,
            // b6
            Key::Ctrl,
            // b7
            Key::Alt,
            // b8
            Key::F5,
            // b9
            Key::V,
            // b10
            Key::F3,
            // b11
            Key::Escape,
        ],
        //joystick: JoystickConfig::Keys {
        //    x_axis: AxisKeyConfig {
        //        positive: Key::F8,
        //        negative: Key::F7,
        //        deadzone: 0.35,
        //    },
        //    y_axis: AxisKeyConfig {
        //        positive: Key::W,
        //        negative: Key::S,
        //        deadzone: 0.40,
        //    }
        //}
        joystick: JoystickConfig::Mouse {
            x_axis: AxisMouseConfig {
                mouse_mode: MouseMode::Constant(2),
                deadzone: 0.25,
            },
            y_axis: AxisMouseConfig {
                mouse_mode: MouseMode::Constant(-2),
                deadzone: 0.20,
            }
        }
    };

    let mut manager = Manager::new(cfg);
    // let mut manager = Manager::dbg();
    let hidapi = HidApi::new()?;

    println!("Attempting to open the Attack3...");
    let attack3 = hidapi.open(VID, PID)?;
    attack3.set_blocking_mode(false)?;
    println!("Opened the Attack3");

    println!("Attempting to read from the Attack3...");

    let mut zeroed = false;
    let mut buffer = [0u8; 1024];
    let mut zero = [0, 0];
    let mut s: Option<State> = None;
    loop {
        // TODO: make polling rate configurable
        thread::sleep(time::Duration::from_millis(3));
        if let Some(ref s) = s {
            manager.step(s.clone());
        }

        let read_len = attack3.read(&mut buffer);
        if let Ok(read_len) = read_len {
            if read_len == 0 {
                continue;
            }
            let packet_count = read_len / PACKET_LENGTH;
            // println!("Received {} packets", packet_count);
            for i in 0..packet_count {
                let start = i * PACKET_LENGTH;
                let p = Packet::parse(&buffer[start..start + PACKET_LENGTH])?;

                if !zeroed {
                    zero[0] = p.x_axis;
                    zero[1] = p.y_axis;
                    zeroed = true;
                }
                s = Some(State::from_packet(zero, p));
                // println!("{}", p);
                // println!("{}", s);
            }
        }
    }
}
