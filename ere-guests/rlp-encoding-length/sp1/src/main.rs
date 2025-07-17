//! SP1 rlp encoding length benchmark

#![no_main]

use std::ops::Deref;

use reth_ethereum_primitives::Block;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let block = sp1_zkvm::io::read::<BincodeBlock>();
    let iterations = sp1_zkvm::io::read::<u16>();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: rlp_encoding");
    for _ in 0..iterations {
        Block::rlp_length_for(&block.header, &block.body);
    }
    println!("cycle-tracker-report-end: rlp_encoding");
}

/// Block wrapper that supports bincode serialization
/// NOTE: Very soon this definition will disappear when guests-lib is used as a workspace dependency.
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BincodeBlock(
    #[serde_as(
        as = "reth_primitives_traits::serde_bincode_compat::Block<reth_ethereum_primitives::TransactionSigned, alloy_consensus::Header>"
    )]
    pub Block,
);

impl Deref for BincodeBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
