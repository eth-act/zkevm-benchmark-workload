//! OpenVM guest program

use ere_platform_openvm::OpenVMPlatform;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use sha2::Sha256;

openvm::init!();

/// Entry point.
pub fn main() {
    openvm_revm_crypto::install_openvm_crypto()
        .expect("failed to install OpenVM revm crypto provider");
    RethStatelessValidatorGuest::run::<OpenVMPlatform<Sha256>>();
}
