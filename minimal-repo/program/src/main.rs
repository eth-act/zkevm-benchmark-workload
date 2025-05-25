//! SP1 guest program

#![no_main]

extern crate alloc;

use alloc::sync::Arc;

use reth_stateless::{fork_spec::ForkSpec, validation::stateless_validation, ClientInput};

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    let input = sp1_zkvm::io::read::<ClientInput>();
    let fork_spec = sp1_zkvm::io::read::<ForkSpec>();
    let chain_spec = Arc::new(fork_spec.into());

    stateless_validation(input.block, input.witness, chain_spec).unwrap();
}
