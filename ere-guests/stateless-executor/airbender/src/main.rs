//! Airbender guest program for stateless execution (no validation).

#![no_std]
#![no_main]
#![no_builtins]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(generic_const_exprs)]

use ere_platform_airbender::AirbenderPlatform;
use reth_stateless_executor::guest::{Guest, RethStatelessExecutorGuest};

mod airbender_rt;

/// Entry point.
pub fn main() {
    RethStatelessExecutorGuest::run_output_sha256::<AirbenderPlatform>();
}

