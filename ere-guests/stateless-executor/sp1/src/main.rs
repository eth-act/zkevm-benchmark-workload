//! SP1 guest program for stateless execution (no validation).

#![no_main]

use ere_platform_sp1::{sp1_zkvm, SP1Platform};
use reth_stateless_executor::guest::{Guest, RethStatelessExecutorGuest};
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);

/// Entry point.
pub fn main() {
    init_tracing_just_like_println();
    RethStatelessExecutorGuest::run_output_sha256::<SP1Platform>();
}

/// Initializes a basic `tracing` subscriber that mimics `println!` behavior.
fn init_tracing_just_like_println() {
    let plain = fmt::format()
        .without_time()
        .with_level(false)
        .with_target(false);

    fmt::Subscriber::builder()
        .event_format(plain)
        .with_writer(std::io::stdout)
        .with_max_level(tracing::Level::INFO)
        .init();
}

