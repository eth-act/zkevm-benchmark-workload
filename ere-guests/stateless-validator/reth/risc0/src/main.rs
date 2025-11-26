//! Risc0 guest program

extern crate alloc;

use ere_platform_risc0::Risc0Platform;
use reth_guest::guest::ethereum_guest;
use sha2::Sha256;

/// Entry point.
pub fn main() {
    ethereum_guest::<Risc0Platform<Sha256>>();
}
