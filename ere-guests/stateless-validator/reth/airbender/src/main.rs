//! Airbender guest program

#![no_std]
#![no_main]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]

use ere_platform_airbender::AirbenderPlatform;
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};

mod airbender_rt;

/// Entry point.
pub fn main() {
    RethStatelessValidatorGuest::run_output_sha256::<AirbenderPlatform>();
}
