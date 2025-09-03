//! OpenVM guest program

use openvm::io::{read, reveal_bytes32};
use reth_stateless::{fork_spec::ForkSpec, validation::stateless_validation, ClientInput};
use std::sync::Arc;
// Imports needed by the linker, but clippy can't tell:
#[allow(unused_imports, clippy::single_component_path_imports)]
use {
    k256::Secp256k1Point,
    openvm_algebra_guest::IntMod,
    openvm_keccak256_guest, // trigger extern native-keccak256
    openvm_pairing::{bls12_381::Bls12_381G1Affine, bn254::Bn254G1Affine},
};

// Initialize modular arithmetic and elliptic curve opcodes.
openvm::init!();

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: ClientInput = read();
    let fork_spec: ForkSpec = read();
    let chain_spec = Arc::new(fork_spec.into());
    println!("end read_input");

    println!("start validation");
    let block_hash = stateless_validation(input.block, input.witness, chain_spec).unwrap();
    println!("end validation");

    // Reveal the block hash.
    reveal_bytes32(*block_hash);
}
