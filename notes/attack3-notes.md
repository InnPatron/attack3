Device: `046d:c214`
IN Endpoint address: `0x81`

Default device state: `0x827f000000`

Packets come in 5-byte segments: `0xXXYYZZCCCC`
* X: x-axis (range: `0x00-0xFF`; `0x00` corresponds to fully left)
* Y: y-axis (range: `0x00-0xFF`; `0x00` corresponds to fully forward)
* Z: z-axis (range: `0x00-0xFF`; `0x00` corresponds to fully up)
* C: button bits (`0` => not pressed)

Button 1:
`0x0100` => `0000_0001_0000_0000`

Button 2:
`0x0200` => `0000_0010_0000_0000`

Button 3:
`0x0400` => `0000_0100_0000_0000`

Button 4:
`0x0800` => `0000_1000_0000_0000`

Button 5:
`0x1000` => `0001_0000_0000_0000`

Button 6:
`0x2000` => `0010_0000_0000_0000`

Button 7:
`0x4000` => `0100_0000_0000_0000`

Button 8:
`0x8000` => `1000_0000_0000_0000`

Button 9:
`0x0001` => `0000_0000_0000_0001`

Button 10:
`0x0002` => `0000_0000_0000_0010`

Button 11:
`0x0004` => `0000_0000_0000_0100`
