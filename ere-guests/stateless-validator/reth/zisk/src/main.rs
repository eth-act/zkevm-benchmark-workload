//! ZisK guest program

#![no_main]

use ere_platform_zisk::{ZiskPlatform, ziskos};
use reth_guest::guest::ethereum_guest;
use sha2::Sha256;

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    ethereum_guest::<ZiskPlatform<Sha256>>();
}
