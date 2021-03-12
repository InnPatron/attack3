extern crate hidapi;
#[cfg(target_os = "windows")]
extern crate bindings;

use std::fs::File;
use std::io::BufReader;
use std::env;
use std::{thread, time};
use std::error::Error;

use hidapi::{HidApi};
use serde_json;

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
    let cfg_path = env::args()
        .skip(1)
        .next()
        .expect("No JSON config path");

    println!("Attempting to JSON config from '{}'", cfg_path);

    let f = File::open(cfg_path)?;
    let cfg: Config = serde_json::from_reader(BufReader::new(f))?;

    println!("Read JSON config");


    let mut manager = Manager::new(cfg);
    // let mut manager = Manager::dbg();
    let hidapi = HidApi::new()?;

    println!("Attempting to open the Attack3...");
    let attack3 = hidapi.open(VID, PID)?;
    attack3.set_blocking_mode(false)?;
    println!("Opened the Attack3");

    println!("Attempting to read from the Attack3 with polling delay {}...",
             cfg.polling_delay);

    let mut zeroed = false;
    let mut buffer = [0u8; 1024];
    let mut zero = [0, 0];
    let mut s: Option<State> = None;
    loop {
        // TODO: make polling rate configurable
        thread::sleep(time::Duration::from_millis(cfg.polling_delay));
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
