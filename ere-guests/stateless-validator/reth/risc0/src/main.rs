//! Risc0 guest program

use ere_platform_risc0::Risc0Platform;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::run_output_sha256::<Risc0Platform>();
}
