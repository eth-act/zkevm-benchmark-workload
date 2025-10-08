//! ZisK guest program

#![no_main]

use std::io::Cursor;

use ere_reth_guest::{
    guest::ethereum_guest,
    sdk::{PublicInputs, ScopeMarker, SDK},
};
use k256::ecdsa::VerifyingKey;
use reth_stateless::StatelessInput;
use sha2::{Digest, Sha256};

ziskos::entrypoint!(main);

#[allow(missing_debug_implementations)]
struct ZiskSDK;

impl SDK for ZiskSDK {
    fn read_inputs() -> (StatelessInput, Vec<VerifyingKey>) {
        let mut input_bytes = Cursor::new(ziskos::read_input());
        let input: StatelessInput = bincode::deserialize_from(&mut input_bytes).unwrap();
        let public_keys: Vec<VerifyingKey> = bincode::deserialize_from(&mut input_bytes).unwrap();
        (input, public_keys)
    }

    fn commit_outputs(pi: &PublicInputs) {
        let public_inputs = (
            pi.block_hash,
            pi.parent_hash,
            pi.withdrawals_root,
            pi.versioned_hashes_hash,
            pi.parent_beacon_block_root,
            pi.requests_hash,
            pi.is_valid,
        );
        let public_inputs_hash = Sha256::digest(bincode::serialize(&public_inputs).unwrap());
        public_inputs_hash
            .chunks_exact(4)
            .enumerate()
            .for_each(|(idx, bytes)| {
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
