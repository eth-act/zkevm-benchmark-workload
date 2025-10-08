//! Pico guest program

#![no_main]

extern crate alloc;

use ere_reth_guest::{
    guest::ethereum_guest,
    sdk::{PublicInputs, ScopeMarker, SDK},
};
use k256::ecdsa::VerifyingKey;
use pico_sdk::io::{commit, read_as};
use reth_stateless::StatelessInput;

pico_sdk::entrypoint!(main);

struct PicoSDK;

impl SDK for PicoSDK {
    fn read_inputs() -> (StatelessInput, Vec<VerifyingKey>) {
        let input = read_as();
        let public_keys = read_as();
        (input, public_keys)
    }

    fn commit_outputs(pi: &PublicInputs) {
        commit(&pi.block_hash);
        commit(&pi.parent_hash);
        commit(&pi.withdrawals_root);
        commit(&pi.versioned_hashes_hash);
        commit(&pi.parent_beacon_block_root);
        commit(&pi.requests_hash);
        commit(&pi.is_valid);
    }

    fn cycle_scope(scope: ScopeMarker, message: &str) {
        match scope {
            ScopeMarker::Start => {
                println!("cycle-tracker-report-start: {message}")
            }
            ScopeMarker::End => {
                println!("cycle-tracker-report-end: {message}")
            }
        }
    }
}

/// Entry point.
pub fn main() {
    ethereum_guest::<PicoSDK>();
}
