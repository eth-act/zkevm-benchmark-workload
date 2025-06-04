//! ZisK guest program

#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use reth_stateless::{ClientInput, fork_spec::ForkSpec, validation::stateless_validation};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let (input, fork_spec): (ClientInput, ForkSpec) =
        bincode::deserialize(&ziskos::read_input()).unwrap();
    let chain_spec = Arc::new(fork_spec.into());
    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec).unwrap();
    println!("end validation");
}
