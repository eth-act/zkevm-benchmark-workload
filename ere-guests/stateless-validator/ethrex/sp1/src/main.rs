#![no_main]

use ere_platform_sp1::{Platform, SP1Platform, sp1_zkvm};
use guest_program::{execution::execution_program, input::ProgramInput};
use k256::sha2::{Digest, Sha256};
use rkyv::rancor::Error;

type P = SP1Platform;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    P::cycle_scope_start("read_input");
    let input = P::read_whole_input();
    let mut input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();
    P::cycle_scope_end("read_input");

    P::cycle_scope_start("public_inputs_preparation");
    let block_hash = input.blocks[0].hash();
    // Ethrex forces the block hash of any provided block to be zero, instead of accepting pre-calculated values
    // and just checking them again.
    input.blocks[0].header.hash = Default::default();
    let parent_hash = input.blocks[0].header.parent_hash;
    P::cycle_scope_end("public_inputs_preparation");

    P::cycle_scope_start("validation");
    if input.blocks.len() != 1 {
        commit_output(block_hash.0, parent_hash.0, false);
        return;
    }
    let res = execution_program(input);
    P::cycle_scope_end("validation");

    P::cycle_scope_start("commit_public_inputs");
    match res {
        Ok(out) => {
            commit_output(out.last_block_hash.0, parent_hash.0, true);
        }
        Err(_) => {
            commit_output(block_hash.0, parent_hash.0, false);
        }
    }
    P::cycle_scope_end("commit_public_inputs");
}

fn commit_output(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool) {
    let public_inputs = (block_hash, parent_hash, is_valid);
    let public_inputs_hash: [u8; 32] =
        Sha256::digest(bincode::serialize(&public_inputs).unwrap()).into();
    P::write_whole_output(&public_inputs_hash);
}
