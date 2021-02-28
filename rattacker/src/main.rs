extern crate hidapi;
#[cfg(target_os = "windows")]
extern crate bindings;

use std::error::Error;
use std::fmt;

use hidapi::{HidApi};

#[macro_use]
mod dispatch;

#[cfg(target_os = "windows")]
mod win_input;

use dispatch::*;

const PACKET_LENGTH: usize = 5;
const VID: u16 = 0x046d;
const PID: u16 = 0xc214;

fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config {
        buttons: [
            // b1
            Key::K1,
            // b2
            Key::K2,
            // b3
            Key::K3,
            // b4
            Key::K4,
            // b5
            Key::K5,
            // b6
            Key::K6,
            // b7
            Key::Escape,
            // b8
            Key::V,
            // b9
            Key::F9,
            // b10
            Key::F5,
            // b11
            Key::Ctrl,
        ],
        x_axis_positive: Key::F8,
        x_axis_negative: Key::F7,

        y_axis_positive: Key::W,
        y_axis_negative: Key::S,

        x_dead_zone: 0.5,
        y_dead_zone: 0.5,
    };

    let mut manager = Manager::new(cfg);
    // let mut manager = Manager::dbg();
    let hidapi = HidApi::new()?;

    println!("Attempting to open the Attack3...");
    let attack3 = hidapi.open(VID, PID)?;
    println!("Opened the Attack3");

    println!("Attempting to read from the Attack3...");

    let mut zeroed = false;
    let mut buffer = [0u8; 1024];
    let mut zero = [0, 0];
    loop {
        let read_len = attack3.read(&mut buffer);
        if let Ok(read_len) = read_len {
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
                // println!("{}", p);
                let s = State::from_packet(zero, p);
                // println!("{}", s);
                manager.step(s);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub buttons: [bool; 11],
    pub x_axis: u8,
    pub y_axis: u8,
    pub z_axis: u8,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, b) in self.buttons.iter().enumerate() {
            write!(f, "button {}: {}\n", i + 1, b)?;
        }

        write!(f, "x-axis: {:#02X}\n", self.x_axis)?;
        write!(f, "y-axis: {:#02X}\n", self.y_axis)?;
        write!(f, "z-axis: {:#02X}\n", self.z_axis)?;

        Ok(())
    }
}

impl Packet {
    fn parse(b: &[u8]) -> Result<Packet, String> {
        assert!(b.len() == 5);

        let mut packet = Packet {
            buttons: [false; 11],
            x_axis: 0,
            y_axis: 0,
            z_axis: 0,
        };

        packet.x_axis = b[0];
        packet.y_axis = b[1];
        packet.z_axis = b[2];

        let bb1 = b[3];
        let bb2 = b[4];

        //Button 1:
        //`0x0100` => `0000_0001_0000_0000`
        //Button 2:
        //`0x0200` => `0000_0010_0000_0000`
        //Button 3:
        //`0x0400` => `0000_0100_0000_0000`
        //Button 4:
        //`0x0800` => `0000_1000_0000_0000`
        //Button 5:
        //`0x1000` => `0001_0000_0000_0000`
        //Button 6:
        //`0x2000` => `0010_0000_0000_0000`
        //Button 7:
        //`0x4000` => `0100_0000_0000_0000`
        //Button 8:
        //`0x8000` => `1000_0000_0000_0000`
        //Button 9:
        //`0x0001` => `0000_0000_0000_0001`
        //Button 10:
        //`0x0002` => `0000_0000_0000_0010`
        //Button 11:
        //`0x0004` => `0000_0000_0000_0100`

        packet.buttons[0] = bb1 & 0b0000_0001 > 0;
        packet.buttons[1] = bb1 & 0b0000_0010 > 0;
        packet.buttons[2] = bb1 & 0b0000_0100 > 0;
        packet.buttons[3] = bb1 & 0b0000_1000 > 0;

        packet.buttons[4] = bb1 & 0b0001_0000 > 0;
        packet.buttons[5] = bb1 & 0b0010_0000 > 0;
        packet.buttons[6] = bb1 & 0b0100_0000 > 0;
        packet.buttons[7] = bb1 & 0b1000_0000 > 0;

        packet.buttons[8] = bb2 & 0b0000_0001 > 0;
        packet.buttons[9] = bb2 & 0b0000_0010 > 0;
        packet.buttons[10] = bb2 & 0b0000_0100 > 0;

        Ok(packet)
    }
}
