#![no_main]

use rkyv::rancor::Error;
use zkvm_interface::{execution::execution_program, io::ProgramInput};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    // TODO: cycle trackers
    let input = sp1_zkvm::io::read_vec();
    let input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();

    // TODO: assert input.blocks.len() == 1

    let block_hash = input.blocks[0].hash();
    let parent_hash = input.blocks[0].header.parent_hash;

    let res = execution_program(input);

    match res {
        Ok(out) => {
            sp1_zkvm::io::commit(&out.last_block_hash);
            sp1_zkvm::io::commit(&parent_hash);
            sp1_zkvm::io::commit(&true);
        }
        Err(_) => {
            sp1_zkvm::io::commit(&block_hash);
            sp1_zkvm::io::commit(&parent_hash);
            sp1_zkvm::io::commit(&false);
        }
    }
}
