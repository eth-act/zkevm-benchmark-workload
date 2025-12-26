//! Risc0 guest program for stateless execution (no validation).

use ere_platform_risc0::Risc0Platform;
use reth_stateless_executor::guest::{Guest, RethStatelessExecutorGuest};

/// Entry point.
pub fn main() {
    RethStatelessExecutorGuest::run_output_sha256::<Risc0Platform>();
}

