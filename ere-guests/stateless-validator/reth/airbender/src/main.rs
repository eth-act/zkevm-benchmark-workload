//! Airbender guest program

#![no_std]
#![no_main]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]

use ere_platform_airbender::AirbenderPlatform;
use reth_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};
use sha2::Sha256;

mod airbender_rt;

#[allow(missing_debug_implementations)]
struct AirbenderSDK;

impl SDK for AirbenderSDK {
    type Platform = AirbenderPlatform<Sha256>;

    fn cycle_scope(_: ScopeMarker, _: &str) {}
}

/// Entry point.
pub fn main() {
    ethereum_guest::<AirbenderSDK>();
}
