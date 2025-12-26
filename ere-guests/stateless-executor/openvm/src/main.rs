//! OpenVM guest program for stateless execution (no validation).

use ere_platform_openvm::OpenVMPlatform;
use reth_stateless_executor::guest::{Guest, RethStatelessExecutorGuest};

openvm::init!();

/// Entry point.
pub fn main() {
    openvm_revm_crypto::install_openvm_crypto()
        .expect("failed to install OpenVM revm crypto provider");
    RethStatelessExecutorGuest::run_output_sha256::<OpenVMPlatform>();
}

