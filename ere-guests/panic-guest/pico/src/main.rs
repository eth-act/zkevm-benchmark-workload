//! Pico panic guest.

#![no_main]

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    panic!("The ticker is eth")
}
