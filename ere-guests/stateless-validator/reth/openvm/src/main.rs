//! OpenVM guest program

use ere_platform_openvm::OpenVMPlatform;
use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};
use sha2::Sha256;

// For linker declarations:
use openvm_keccak256 as _;

openvm::init!();

struct OpenVMSDK;

impl SDK for OpenVMSDK {
    type Platform = OpenVMPlatform<Sha256>;

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<OpenVMSDK>();
}
