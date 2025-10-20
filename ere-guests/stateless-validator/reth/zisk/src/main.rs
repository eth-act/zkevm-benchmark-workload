//! ZisK guest program

#![no_main]

use reth_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};

ziskos::entrypoint!(main);

#[allow(missing_debug_implementations)]
struct ZiskSDK;

impl SDK for ZiskSDK {
    fn read_input() -> Vec<u8> {
        ziskos::read_input()
    }

    fn commit_output(output: [u8; 32]) {
        output.chunks_exact(4).enumerate().for_each(|(idx, bytes)| {
            ziskos::set_output(idx, u32::from_le_bytes(bytes.try_into().unwrap()))
        });
    }

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
