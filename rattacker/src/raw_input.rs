use std::fmt;

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
    pub fn parse(b: &[u8]) -> Result<Packet, String> {
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
