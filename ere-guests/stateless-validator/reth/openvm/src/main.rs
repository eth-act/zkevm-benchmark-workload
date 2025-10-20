//! OpenVM guest program

use openvm::io::{read_vec, reveal_bytes32};
use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};

// For linker declarations:
use openvm_keccak256 as _;

openvm::init!();

struct OpenVMSDK;

impl SDK for OpenVMSDK {
    fn read_input() -> Vec<u8> {
        read_vec()
    }

    fn commit_output(output: [u8; 32]) {
        reveal_bytes32(output);
    }

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<OpenVMSDK>();
}
