#![no_main]

use guest_program::{execution::execution_program, input::ProgramInput};
use k256::sha2::{Digest, Sha256};
use rkyv::rancor::Error;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read_vec();
    let mut input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: public_inputs_preparation");
    let block_hash = input.blocks[0].hash();
    // Ethrex forces the block hash of any provided block to be zero, instead of accepting pre-calculated values
    // and just checking them again.
    input.blocks[0].header.hash = Default::default();
    let parent_hash = input.blocks[0].header.parent_hash;
    println!("cycle-tracker-report-end: public_inputs_preparation");

    println!("cycle-tracker-report-start: validation");
    if input.blocks.len() != 1 {
        commit_output(block_hash.0, parent_hash.0, false);
        return;
    }
    let res = execution_program(input);
    println!("cycle-tracker-report-end: validation");

    println!("cycle-tracker-report-start: commit_public_inputs");
    match res {
        Ok(out) => {
            commit_output(out.last_block_hash.0, parent_hash.0, true);
        }
        Err(_) => {
            commit_output(block_hash.0, parent_hash.0, false);
        }
    }
    println!("cycle-tracker-report-end: commit_public_inputs");
}

fn commit_output(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool) {
    let public_inputs = (block_hash, parent_hash, is_valid);
    let public_inputs_hash: [u8; 32] =
        Sha256::digest(bincode::serialize(&public_inputs).unwrap()).into();
    sp1_zkvm::io::commit(&public_inputs_hash);
}
