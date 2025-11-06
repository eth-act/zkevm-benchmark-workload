//! SP1 block encoding length benchmark

#![no_main]

use ere_platform_sp1::sp1_zkvm;

sp1_zkvm::entrypoint!(main);

use block_encoding_length_io::{BlockEncodingFormat, Input, io_serde};
use ere_io_serde::IoSerde;
use guest_libs::block_ssz;
use reth_ethereum_primitives::Block;
use ssz::Encode;

pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let input_bytes = sp1_zkvm::io::read_vec();
    let input: Input = io_serde()
        .deserialize(&input_bytes)
        .expect("Deserialization failed");
    println!("cycle-tracker-report-end: read_input");

    match input.format {
        BlockEncodingFormat::Rlp => {
            println!("cycle-tracker-report-start: block_encoding_length_calculation");
            for _ in 0..input.loop_count {
                Block::rlp_length_for(&input.block.header, &input.block.body);
            }
            println!("cycle-tracker-report-end: block_encoding_length_calculation");
        }
        BlockEncodingFormat::Ssz => {
            println!("cycle-tracker-report-start: block_format_conversion");
            let block: block_ssz::Block = input.block.0.into();
            println!("cycle-tracker-report-end: block_format_conversion");
            println!("cycle-tracker-report-start: block_encoding_length_calculation");
            for _ in 0..input.loop_count {
                block.ssz_bytes_len();
            }
            println!("cycle-tracker-report-end: block_encoding_length_calculation");
        }
    }
}
