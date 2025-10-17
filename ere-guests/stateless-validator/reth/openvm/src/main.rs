//! OpenVM guest program

use ere_reth_guest::{
    guest::ethereum_guest,
    sdk::{PublicInputs, ScopeMarker, SDK},
};
use guest_libs::senders::UncompressedPublicKey;
use openvm::io::{read, reveal_bytes32};
use sha2::{Digest, Sha256};

// For linker declarations:
use openvm_keccak256 as _;
use reth_stateless::StatelessInput;

openvm::init!();

struct OpenVMSDK;

impl SDK for OpenVMSDK {
    fn read_inputs() -> (StatelessInput, Vec<UncompressedPublicKey>) {
        let input = read();
        let public_keys: Vec<UncompressedPublicKey> = read();
        (input, public_keys)
    }

    fn commit_outputs(pi: &PublicInputs) {
        let public_inputs = (pi.block_hash, pi.parent_hash, pi.is_valid);
        let public_inputs_hash = Sha256::digest(bincode::serialize(&public_inputs).unwrap());
        reveal_bytes32(public_inputs_hash.into());
    }

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<OpenVMSDK>();
}
