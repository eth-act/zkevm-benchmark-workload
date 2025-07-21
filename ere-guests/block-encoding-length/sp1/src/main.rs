//! SP1 block encoding length benchmark

#![no_main]
sp1_zkvm::entrypoint!(main);

use guest_libs::block_ssz;
use reth_ethereum_primitives::Block;
use ssz::Encode;

const RLP_FORMAT: u8 = 0;
const SSZ_FORMAT: u8 = 1;

pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    // Read the block to encode.
    let block = sp1_zkvm::io::read::<guest_libs::BincodeBlock>();
    // Read the number of iterations to perform in order to amortize fixed costs.
    let iterations = sp1_zkvm::io::read::<u16>();
    // Supported formats: 0: RLP, 1: SSZ
    let format = sp1_zkvm::io::read::<u8>();
    println!("cycle-tracker-report-end: read_input");

    match format {
        RLP_FORMAT => {
            println!("cycle-tracker-report-start: rlp_encoding_length");
            for _ in 0..iterations {
                Block::rlp_length_for(&block.header, &block.body);
            }
            println!("cycle-tracker-report-end: rlp_encoding_length");
        }
        SSZ_FORMAT => {
            println!("cycle-tracker-report-start: format_conversion");
            let block: block_ssz::Block = block.0.into();
            println!("cycle-tracker-report-end: format_conversion");
            println!("cycle-tracker-report-start: ssz_encoding_length");
            for _ in 0..iterations {
                block.ssz_bytes_len();
            }
            println!("cycle-tracker-report-end: ssz_encoding_length");
        }
        _ => panic!("Unsupported format"),
    }
}
