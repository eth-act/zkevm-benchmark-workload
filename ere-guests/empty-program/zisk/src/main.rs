//! ZisK guest program
#![no_main]

use ere_platform_zisk::ziskos;

ziskos::entrypoint!(main);

/// Entry point
pub fn main() {}
