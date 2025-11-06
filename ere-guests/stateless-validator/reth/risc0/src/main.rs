//! Risc0 guest program

extern crate alloc;

use ere_platform_risc0::Risc0Platform;
use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};
use sha2::Sha256;

pub struct Risc0SDK;

impl SDK for Risc0SDK {
    type Platform = Risc0Platform<Sha256>;

    fn cycle_scope(_scope: ScopeMarker, _message: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<Risc0SDK>();
}
