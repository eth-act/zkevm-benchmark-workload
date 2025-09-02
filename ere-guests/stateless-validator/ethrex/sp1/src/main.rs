#![no_main]

use rkyv::rancor::Error;
use zkvm_interface::{execution::execution_program, io::ProgramInput};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read_vec();
    let input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: public_inputs_preparation");
    let block_hash = input.blocks[0].hash();
    let parent_hash = input.blocks[0].header.parent_hash;
    println!("cycle-tracker-report-end: public_inputs_preparation");

    println!("cycle-tracker-report-start: validation");
    if input.blocks.len() != 1 {
        sp1_zkvm::io::commit(&block_hash.0);
        sp1_zkvm::io::commit(&parent_hash.0);
        sp1_zkvm::io::commit(&false);
    }
    let res = execution_program(input);
    println!("cycle-tracker-report-end: validation");

    println!("cycle-tracker-report-start: commit_public_inputs");
    match res {
        Ok(out) => {
            sp1_zkvm::io::commit(&out.last_block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&true);
        }
        Err(_) => {
            sp1_zkvm::io::commit(&block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&false);
        }
    }
    println!("cycle-tracker-report-end: commit_public_inputs");
}
