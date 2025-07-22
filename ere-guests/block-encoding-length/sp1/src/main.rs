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
            println!("cycle-tracker-report-start: block_encoding_length_calculation");
            for _ in 0..iterations {
                Block::rlp_length_for(&block.header, &block.body);
            }
            println!("cycle-tracker-report-end: block_encoding_length_calculation");
        }
        SSZ_FORMAT => {
            println!("cycle-tracker-report-start: block_format_conversion");
            let block: block_ssz::Block = block.0.into();
            println!("cycle-tracker-report-end: block_format_conversion");
            println!("cycle-tracker-report-start: block_encoding_length_calculation");
            for _ in 0..iterations {
                block.ssz_bytes_len();
            }
            println!("cycle-tracker-report-end: block_encoding_length_calculation");
        }
        _ => panic!("Unsupported format"),
    }
}
