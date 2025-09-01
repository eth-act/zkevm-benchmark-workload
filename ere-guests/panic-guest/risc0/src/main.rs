//! Risc0 panic guest program

extern crate alloc;

use risc0_zkvm::guest::env;

/// Entry point.
pub fn main() {
    panic!("The ticker is eth")
}
