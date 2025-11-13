//! OpenVM guest program

use openvm::io::{read_vec, reveal_bytes32};
use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};

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
    openvm_revm_crypto::install_openvm_crypto()
        .expect("failed to install OpenVM revm crypto provider");
    ethereum_guest::<OpenVMSDK>();
}
