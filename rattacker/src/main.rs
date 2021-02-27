use std::ptr;

use libusb_sys::{
    libusb_device as Device,
    libusb_device_descriptor as DeviceDescriptor,
    self as libusb,
};

use libc::{c_int};

macro_rules! dflt_ctxt {
    () => (ptr::null_mut())
}

fn main() {
    let mut devices: *const *mut Device = ptr::null_mut();
    unsafe {

        // Initialize libusb default context
        println!("Initializing libusb...");
        let status: c_int = libusb::libusb_init(dflt_ctxt!());
        if status != 0 {
            eprintln!("Error initializing: {}", status);
            return;
        }
        println!("Initialized libusb");

        println!("Getting device list");
        let len = libusb::libusb_get_device_list(dflt_ctxt!(), &mut devices as *mut _);
        if len > 0 {
            let result = print_devices(devices);
            if result.is_err() {
                eprintln!("Error printing device list");
                libusb::libusb_exit(dflt_ctxt!());
                return;
            }

        } else {
            eprintln!("Error. Device list length: {}", len);
            libusb::libusb_exit(dflt_ctxt!());
            return;
        }
        libusb::libusb_exit(dflt_ctxt!());
    }
}

unsafe fn print_devices(devices: *const *mut Device) -> Result<(), c_int> {
    let mut dev: *mut Device = *devices;
    let mut i = 0;

    while dev != ptr::null_mut() {
        let mut desc = DeviceDescriptor {
            bLength: 0,
            bDescriptorType: 0,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        };
        let status = libusb::libusb_get_device_descriptor(dev, &mut desc as *mut _);
        if status != 0 {
            eprintln!("Failed to get device descriptor");
            return Err(status);
        }

        let bus_number =  libusb::libusb_get_bus_number(dev);
        let device_addr = libusb::libusb_get_device_address(dev);

        println!("{:#06x}:{:#06x} (bus {}, device {})",
            desc.idVendor, desc.idProduct, bus_number, device_addr);

        let mut path = [0u8; 8];
        let status = libusb::libusb_get_port_numbers(dev, &mut path[0] as *mut _, path.len() as c_int);
        if status > 0 {
            println!("path: {}", path[0]);
            for s in path.iter() {
                print!(".{}", s);
            }
        }
        print!("\n");
        i = i + 1;
        dev = *devices.offset(i);
    }
    Ok(())
}
