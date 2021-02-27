Device: `046d:c214`
IN Endpoint address: `0x81`

Default device state: `0x827f000000`

Packets come in 10-byte segments: `0xXXYYZZCCCC`
* X: x-axis (range: `0x00-0xFF`; `0x00` corresponds to fully left)
* Y: y-axis (range: `0x00-0xFF`; `0x00` corresponds to fully forward)
* Z: z-axis (range: `0x00-0xFF`; `0x00` corresponds to fully up)
* C: button bits (`0` => not pressed)

Button 1:
`0x0100`

Button 2:
`0x0200`

Button 3:
`0x0400`

Button 4:
`0x0800`

Button 5:
`0x1000`

Button 6:
`0x2000`

Button 7:
`0x4000`

Button 8:
`0x8000`

Button 9:
`0x0001`

Button 10:
`0x0002`

Button 11:
`0x0004`
