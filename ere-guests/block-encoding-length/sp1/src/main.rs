//! SP1 block encoding length benchmark

#![no_main]

use block_encoding_length_guest::guest::{BlockEncodingLengthGuest, Guest};
use ere_platform_sp1::{SP1Platform, sp1_zkvm};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    BlockEncodingLengthGuest::run::<SP1Platform>();
}
