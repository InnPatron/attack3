# rattacker

User-space program that translates HID input from a Logitech Attack3 using libhidapi into keyboard and mouse input.

Currently Windows only.

Tested with Rust 1.50 but can probably use earlier versions.

Takes a path to a JSON configuration file.
* See `rattacker/joy-config` for examples
* See `rattacker/src/config.rs` for details

The program is split into four parts:
1. Main event loop (`main.rs`)
2. HID input parsing (`raw_input.rs`)
3. `Manager` handles input state and fires events depending on joystick input (`dispatch.rs`)
4. Input dispatchers (`win_input.rs` for Windows) that perform the actual translation per platform
