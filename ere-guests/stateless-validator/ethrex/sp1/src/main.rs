#![no_main]

use ere_platform_sp1::{sp1_zkvm, SP1Platform};
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, Guest};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    EthrexStatelessValidatorGuest::run_output_sha256::<SP1Platform>();
}
