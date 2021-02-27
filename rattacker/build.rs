extern crate pkg_config;

fn main() {
    pkg_config::probe_library("libusb-1.0").unwrap();
}
