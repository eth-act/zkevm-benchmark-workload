//! Airbender guest program

#![no_std]
#![no_main]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]

use ere_platform_airbender::AirbenderPlatform;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use sha2::Sha256;

mod airbender_rt;

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::run::<AirbenderPlatform<Sha256>>();
}
