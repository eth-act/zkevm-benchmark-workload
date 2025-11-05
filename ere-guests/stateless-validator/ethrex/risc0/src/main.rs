use k256::sha2::{Digest, Sha256};
use risc0_zkvm::guest::env;

use guest_program::{execution::execution_program, input::ProgramInput};
use rkyv::rancor::Error;

fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let len = {
        let mut bytes = [0; 4];
        env::read_slice(&mut bytes);
        u32::from_le_bytes(bytes)
    };
    let mut input = vec![0u8; len as usize];
    env::read_slice(&mut input);
    let mut input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();
    let end = env::cycle_count();
    eprintln!("reading input (cycle tracker): {}", end - start);

    println!("public inputs preparation");
    let start = env::cycle_count();
    let block_hash = input.blocks[0].hash();
    // Ethrex forces the block hash of any provided block to be zero, instead of accepting pre-calculated values
    // and just checking them again.
    input.blocks[0].header.hash = Default::default();
    let parent_hash = input.blocks[0].header.parent_hash;
    let end = env::cycle_count();
    eprintln!("public inputs preparation (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
    if input.blocks.len() != 1 {
        commit_output(block_hash.0, parent_hash.0, false);
        return;
    }
    let res = execution_program(input);
    let end = env::cycle_count();
    eprintln!("stateless validation (cycle tracker): {}", end - start);

    println!("start commit public inputs");
    let start = env::cycle_count();
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    match res {
        Ok(out) => {
            commit_output(out.last_block_hash.0, parent_hash.0, true);
        }
        Err(_) => {
            commit_output(block_hash.0, parent_hash.0, false);
        }
    }
    let end = env::cycle_count();
    eprintln!("commit public inputs (cycle tracker): {}", end - start);
}

fn commit_output(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool) {
    let public_inputs = (block_hash, parent_hash, is_valid);
    let public_inputs_hash: [u8; 32] =
        Sha256::digest(bincode::serialize(&public_inputs).unwrap()).into();
    env::commit_slice(&public_inputs_hash);
}
