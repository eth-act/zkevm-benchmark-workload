use risc0_zkvm::guest::env;

use guest_program::{execution::execution_program, input::ProgramInput};
use rkyv::rancor::Error;

fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let input = env::read_frame();
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
        env::commit(&block_hash.0);
        env::commit(&parent_hash.0);
        env::commit(&false);
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
            env::commit(&out.last_block_hash.0);
            env::commit(&parent_hash.0);
            env::commit(&true);
        }
        Err(_) => {
            env::commit(&block_hash.0);
            env::commit(&parent_hash.0);
            env::commit(&false);
        }
    }
    let end = env::cycle_count();
    eprintln!("commit public inputs (cycle tracker): {}", end - start);
}
