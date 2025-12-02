#![no_main]

use ere_platform_sp1::{sp1_zkvm, SP1Platform};
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, Guest};
use k256::sha2::Sha256;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    EthrexStatelessValidatorGuest::run::<SP1Platform<Sha256>>();
}
