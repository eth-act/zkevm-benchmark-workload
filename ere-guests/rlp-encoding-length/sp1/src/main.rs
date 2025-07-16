//! SP1 rlp encoding length benchmark

#![no_main]

use alloy_consensus::Block;

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let block = sp1_zkvm::io::read::<reth_ethereum_primitives::Block>();
    let iterations = sp1_zkvm::io::read::<u16>();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: rlp_encoding");
    for _ in 0..iterations {
        Block::rlp_length_for(&block.header, &block.body);
    }
    println!("cycle-tracker-report-end: rlp_encoding");
}
