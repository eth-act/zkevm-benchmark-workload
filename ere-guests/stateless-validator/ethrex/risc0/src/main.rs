use ere_platform_risc0::Risc0Platform;
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, Guest};
use k256::sha2::Sha256;

pub fn main() {
    EthrexStatelessValidatorGuest::run::<Risc0Platform<Sha256>>();
}
