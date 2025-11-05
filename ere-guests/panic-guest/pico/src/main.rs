//! Pico panic guest.

#![no_main]

use ere_platform_pico::pico_sdk;

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    panic!("The ticker is eth")
}
