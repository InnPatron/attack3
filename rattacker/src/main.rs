extern crate hidapi;

use std::error::Error;
use std::fmt;

use hidapi::{HidApi};

const PACKET_LENGTH: usize = 5;
const VID: u16 = 0x046d;
const PID: u16 = 0xc214;
const BUTTON_LEN: usize = 11;

macro_rules! printHandler {
    ($msg: expr) => (Box::new(|| { println!("{}", $msg); }))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = Manager {
        previous_state: None,
        button_up: [
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
        button_down: [
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
    };
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

struct Manager {
    previous_state: Option<State>,
    button_up: [Box<dyn Fn() -> ()>; BUTTON_LEN],
    button_down: [Box<dyn Fn() -> ()>; BUTTON_LEN],
}

impl Manager {

    fn step(&mut self, next_state: State) {
        match self.previous_state.take() {
            Some(previous_state) => {
                for i in 0..BUTTON_LEN {
                    let pb = previous_state.buttons[i];
                    let nb = next_state.buttons[i];

                    // println!("{}: {} -> {}", i + 1, pb, nb);

                    if !pb && nb {
                        self.button_down[i]();
                    } else if pb && !nb {
                        self.button_up[i]();
                    }
                }

                self.previous_state = Some(next_state);
            }

            None => {
                self.previous_state = Some(next_state);

            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
struct State {
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
    fn from_packet(zero: [u8; 2], packet: Packet) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    buttons: [bool; 11],
    x_axis: u8,
    y_axis: u8,
    z_axis: u8,
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
