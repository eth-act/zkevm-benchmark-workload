//! Risc0 guest program

use ere_platform_risc0::Risc0Platform;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use sha2::Sha256;

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::run::<Risc0Platform<Sha256>>();
}
