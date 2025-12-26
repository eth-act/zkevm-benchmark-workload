//! ZisK guest program for stateless execution (no validation).

#![no_main]

use ere_platform_zisk::{ziskos, ZiskPlatform};
use reth_stateless_executor::guest::{Guest, RethStatelessExecutorGuest};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    RethStatelessExecutorGuest::run_output_sha256::<ZiskPlatform>();
}

