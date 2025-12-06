use ere_platform_risc0::Risc0Platform;
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, Guest};

pub fn main() {
    EthrexStatelessValidatorGuest::run_output_sha256::<Risc0Platform>();
}
