//! SDK implementation for SP1.
use std::error::Error;

use k256::ecdsa::VerifyingKey;
use reth_stateless::StatelessInput;

use crate::sdk::SDK;

/// SDK implementation for SP1.
#[allow(missing_debug_implementations)]
pub struct SP1SDK;

impl SDK for SP1SDK {
    fn read_inputs() -> Result<(StatelessInput, Vec<VerifyingKey>), Box<dyn Error>> {
        let input = sp1_zkvm::io::read::<StatelessInput>();
        let public_keys = sp1_zkvm::io::read::<Vec<VerifyingKey>>();
        Ok((input, public_keys))
    }

    fn commit_outputs(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool) {
        sp1_zkvm::io::commit(&block_hash);
        sp1_zkvm::io::commit(&parent_hash);
        sp1_zkvm::io::commit(&is_valid);
    }

    fn cycle_scope(scope: super::CycleScope, message: &str) {
        match scope {
            super::CycleScope::Start => {
                println!("cycle-tracker-report-start: {message}")
            }
            super::CycleScope::End => {
                println!("cycle-tracker-report-end: {message}")
            }
        }
    }
}
