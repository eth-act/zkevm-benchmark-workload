//! Risc0 guest program

extern crate alloc;

use std::io::Read;

use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};
use risc0_zkvm::guest::env;

pub struct Risc0SDK;

impl SDK for Risc0SDK {
    fn read_input() -> Vec<u8> {
        let mut input = Vec::new();
        env::stdin()
            .read_to_end(&mut input)
            .expect("Failed to read input");
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
