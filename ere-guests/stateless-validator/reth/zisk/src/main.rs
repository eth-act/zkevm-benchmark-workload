//! ZisK guest program

#![no_main]

use ere_platform_zisk::{ZiskPlatform, ziskos};
use openvm_mpt::statelesstrie::OpenVMStatelessSparseTrie;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::<OpenVMStatelessSparseTrie>::run_output_sha256::<ZiskPlatform>();
}
