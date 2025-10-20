//! SP1 guest program

#![no_main]

extern crate alloc;

use reth_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};
use sp1_zkvm::io::read_vec;
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);

#[allow(missing_debug_implementations)]
struct SP1SDK;

impl SDK for SP1SDK {
    fn read_input() -> Vec<u8> {
        read_vec()
    }

    fn commit_output(output: [u8; 32]) {
        sp1_zkvm::io::commit(&output);
    }

    fn cycle_scope(scope: ScopeMarker, message: &str) {
        match scope {
            ScopeMarker::Start => {
                println!("cycle-tracker-report-start: {message}")
            }
            ScopeMarker::End => {
                println!("cycle-tracker-report-end: {message}")
            }
        }
    }
}

/// Entry point.
pub fn main() {
    init_tracing_just_like_println();
    ethereum_guest::<SP1SDK>();
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
