//! SP1 rlp encoding length benchmark

#![no_main]

use guest_libs::block_ssz::Block;
use ssz::Encode;

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let block = sp1_zkvm::io::read::<guest_libs::BincodeBlock>();
    let iterations = sp1_zkvm::io::read::<u16>();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: format_conversion");
    let block_ssz: Block = block.0.into();
    println!("cycle-tracker-report-end: format_conversion");

    println!("cycle-tracker-report-start: ssz_encoding_length");
    for _ in 0..iterations {
        block_ssz.ssz_bytes_len();
    }
    println!("cycle-tracker-report-end: ssz_encoding_length");
}
