`lsusb`
* Vendor ID:Product ID

`sudo lsusb -vd VID:PID`
* Dumps descriptors
* Response descriptors may be unavailable
* If HID device: driver `usbhid` is claiming it
  * `sudo dmesg | grep usb` to find `X-Y.Z` for the device
  * `sudo bash -c "echo -n X-Y.Z:1.0 >/sys/bus/usb/drivers/usbhid/unbind"`

In order to read from the device without `sudo`, added the following to `/etc/udev/rules.d/80-local.rules:
`SUBSYSTEM=="usb",ATTRS{idVendor}=="046d",ATTRS{idProduct}=="c214",GROUP="alex",MODE="0660"`
