use risc0_zkvm::guest::env;

use guest_program::{execution::execution_program, input::ProgramInput};
use k256::sha2::{Digest, Sha256};
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
    let versioned_hashes_hash: Option<[u8; 32]> = input
        .execution_witness
        .chain_config
        .is_cancun_activated(input.blocks[0].header.timestamp)
        .then_some(
            Sha256::digest(
                input.blocks[0]
                    .body
                    .transactions
                    .iter()
                    .flat_map(|tx| tx.blob_versioned_hashes())
                    .fold(Vec::new(), |mut acc, h| {
                        acc.extend_from_slice(&h.0);
                        acc
                    }),
            )
            .into(),
        );
    let parent_beacon_block_root = input.blocks[0].header.parent_beacon_block_root.map(|h| h.0);
    let requests_hash = input.blocks[0].header.requests_hash.map(|h| h.0);
    let end = env::cycle_count();
    eprintln!("public inputs preparation (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
    if input.blocks.len() != 1 {
        env::commit(&block_hash.0);
        env::commit(&parent_hash.0);
        env::commit(&versioned_hashes_hash);
        env::commit(&parent_beacon_block_root);
        env::commit(&requests_hash);
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
            env::commit(&versioned_hashes_hash);
            env::commit(&parent_beacon_block_root);
            env::commit(&requests_hash);
            env::commit(&true);
        }
        Err(_) => {
            env::commit(&block_hash.0);
            env::commit(&parent_hash.0);
            env::commit(&versioned_hashes_hash);
            env::commit(&parent_beacon_block_root);
            env::commit(&requests_hash);
            env::commit(&false);
        }
    }
    let end = env::cycle_count();
    eprintln!("commit public inputs (cycle tracker): {}", end - start);
}
