//! ZisK panic guest program
#![no_main]
ziskos::entrypoint!(main);

/// Entry point
pub fn main() {
    panic!("The ticker is eth")
}
