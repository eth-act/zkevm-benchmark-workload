//! ZisK guest program

#![no_main]

use std::io::Cursor;

use ere_reth_guest::{
    guest::ethereum_guest,
    sdk::{CycleScope, SDK},
};
use k256::ecdsa::VerifyingKey;
use reth_stateless::StatelessInput;
use sha2::{Digest, Sha256};

ziskos::entrypoint!(main);

/// SDK implementation for Zisk.
#[allow(missing_debug_implementations)]
pub struct ZiskSDK;

impl SDK for ZiskSDK {
    fn read_inputs() -> (StatelessInput, Vec<VerifyingKey>) {
        let mut input_bytes = Cursor::new(ziskos::read_input());
        let input: StatelessInput = bincode::deserialize_from(&mut input_bytes).unwrap();
        let public_keys: Vec<VerifyingKey> = bincode::deserialize_from(&mut input_bytes).unwrap();
        (input, public_keys)
    }

    fn commit_outputs(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool) {
        let public_inputs = (block_hash, parent_hash, is_valid);
        let public_inputs_hash = Sha256::digest(bincode::serialize(&public_inputs).unwrap());
        public_inputs_hash
            .chunks_exact(4)
            .enumerate()
            .for_each(|(idx, bytes)| {
                ziskos::set_output(idx, u32::from_le_bytes(bytes.try_into().unwrap()))
            });
    }

    fn cycle_scope(scope: CycleScope, message: &str) {
        match scope {
            CycleScope::Start => {
                println!("start: {message}")
            }
            CycleScope::End => {
                println!("end: {message}")
            }
        }
    }
}

/// Entry point.
pub fn main() {
    ethereum_guest::<ZiskSDK>();
}
