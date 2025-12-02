//! SP1 guest program

#![no_main]

use ere_platform_sp1::{SP1Platform, sp1_zkvm};
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use sha2::Sha256;
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);

/// Entry point.
pub fn main() {
    init_tracing_just_like_println();
    RethStatelessValidatorGuest::run::<SP1Platform<Sha256>>();
}

/// TODO: can we put this in the host? (Note that if we want sp1 logs, it will look very plain in that case)
/// Initializes a basic `tracing` subscriber that mimics `println!` behavior.
///
/// This is because we want to use tracing in the `no_std` program to capture cycle counts.
fn init_tracing_just_like_println() {
    // Build a formatter that prints *only* the message text + '\n'
    let plain = fmt::format()
        .without_time() // no timestamp
        .with_level(false) // no INFO/TRACE prefix
        .with_target(false); // no module path

    fmt::Subscriber::builder()
        .event_format(plain) // use the stripped-down format
        .with_writer(std::io::stdout) // stdout == println!
        .with_max_level(tracing::Level::INFO) // capture info! and up
        .init(); // set as global default
}
