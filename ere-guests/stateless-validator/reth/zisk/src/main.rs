//! ZisK guest program

#![no_main]

use ere_platform_zisk::{ZiskPlatform, ziskos};
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use sha2::Sha256;

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::run::<ZiskPlatform<Sha256>>();
}
