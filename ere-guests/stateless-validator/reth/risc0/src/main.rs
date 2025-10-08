//! Risc0 guest program

extern crate alloc;

use ere_reth_guest::{
    guest::ethereum_guest,
    sdk::{PublicInputs, ScopeMarker, SDK},
};
use k256::ecdsa::VerifyingKey;
use reth_stateless::StatelessInput;
use risc0_zkvm::guest::env;

pub struct Risc0SDK;

impl SDK for Risc0SDK {
    fn read_inputs() -> (StatelessInput, Vec<VerifyingKey>) {
        let input = env::read::<StatelessInput>();
        let public_keys = env::read::<Vec<VerifyingKey>>();
        (input, public_keys)
    }

    fn commit_outputs(pi: &PublicInputs) {
        env::commit(&pi.block_hash);
        env::commit(&pi.parent_hash);
        env::commit(&pi.is_valid);
    }

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<Risc0SDK>();
}
