use risc0_zkvm::guest::env;

use rkyv::rancor::Error;
use zkvm_interface::{execution::execution_program, io::ProgramInput};

fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let input = env::read_frame();
    let input = rkyv::from_bytes::<ProgramInput, Error>(&input).unwrap();
    let end = env::cycle_count();
    eprintln!("reading input (cycle tracker): {}", end - start);

    println!("public inputs preparation");
    let start = env::cycle_count();
    let block_hash = input.blocks[0].hash();
    let parent_hash = input.blocks[0].header.parent_hash;
    let end = env::cycle_count();
    eprintln!("public inputs preparation (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
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
