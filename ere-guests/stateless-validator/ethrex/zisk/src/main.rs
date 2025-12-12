#![no_main]

use ere_platform_zisk::{ziskos, ZiskPlatform};
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, Guest};

ziskos::entrypoint!(main);

pub fn main() {
    EthrexStatelessValidatorGuest::run_output_sha256::<ZiskPlatform>();
}
