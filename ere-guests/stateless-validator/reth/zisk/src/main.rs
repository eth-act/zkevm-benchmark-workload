//! ZisK guest program

#![no_main]

use ere_platform_zisk::{ZiskPlatform, ziskos};
use reth_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};
use sha2::Sha256;

ziskos::entrypoint!(main);

#[allow(missing_debug_implementations)]
struct ZiskSDK;

impl SDK for ZiskSDK {
    type Platform = ZiskPlatform<Sha256>;

    fn cycle_scope(scope: ScopeMarker, message: &str) {
        match scope {
            ScopeMarker::Start => {
                println!("start: {message}")
            }
            ScopeMarker::End => {
                println!("end: {message}")
            }
        }
    }
}

/// Entry point.
pub fn main() {
    ethereum_guest::<ZiskSDK>();
}
