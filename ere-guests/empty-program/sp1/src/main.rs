//! SP1 empty guest program

#![no_main]

use ere_platform_sp1::sp1_zkvm;

sp1_zkvm::entrypoint!(main);
pub fn main() {}
