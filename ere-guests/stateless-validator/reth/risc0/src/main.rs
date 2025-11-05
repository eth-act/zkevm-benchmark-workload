//! Risc0 guest program

extern crate alloc;

use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};
use risc0_zkvm::guest::env;

pub struct Risc0SDK;

impl SDK for Risc0SDK {
    fn read_input() -> Vec<u8> {
        let len = {
            let mut bytes = [0; 4];
            env::read_slice(&mut bytes);
            u32::from_le_bytes(bytes)
        };
        let mut input = vec![0u8; len as usize];
        env::read_slice(&mut input);
        input
    }

    fn commit_output(output: [u8; 32]) {
        env::commit_slice(&output);
    }

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<Risc0SDK>();
}
