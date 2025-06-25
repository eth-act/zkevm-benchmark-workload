//! OpenVM guest program

use openvm::io::read;

extern crate alloc;

use alloc::sync::Arc;
use reth_stateless::{StatelessInput, fork_spec::ForkSpec, validation::stateless_validation};

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read();
    let fork_spec: ForkSpec = read();
    let chain_spec = Arc::new(fork_spec.into());
    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec).unwrap();
    println!("end validation");
}
