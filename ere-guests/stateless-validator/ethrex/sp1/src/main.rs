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
    let withdrawals_root = input.blocks[0].header.withdrawals_root.map(|h| h.0);
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
    println!("cycle-tracker-report-end: public_inputs_preparation");

    println!("cycle-tracker-report-start: validation");
    if input.blocks.len() != 1 {
        sp1_zkvm::io::commit(&block_hash.0);
        sp1_zkvm::io::commit(&parent_hash.0);
        sp1_zkvm::io::commit(&withdrawals_root);
        sp1_zkvm::io::commit(&versioned_hashes_hash);
        sp1_zkvm::io::commit(&parent_beacon_block_root);
        sp1_zkvm::io::commit(&requests_hash);
        sp1_zkvm::io::commit(&false);
        return;
    }
    let res = execution_program(input);
    println!("cycle-tracker-report-end: validation");

    println!("cycle-tracker-report-start: commit_public_inputs");
    match res {
        Ok(out) => {
            sp1_zkvm::io::commit(&out.last_block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&withdrawals_root);
            sp1_zkvm::io::commit(&versioned_hashes_hash);
            sp1_zkvm::io::commit(&parent_beacon_block_root);
            sp1_zkvm::io::commit(&requests_hash);
            sp1_zkvm::io::commit(&true);
        }
        Err(_) => {
            sp1_zkvm::io::commit(&block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&withdrawals_root);
            sp1_zkvm::io::commit(&versioned_hashes_hash);
            sp1_zkvm::io::commit(&parent_beacon_block_root);
            sp1_zkvm::io::commit(&requests_hash);
            sp1_zkvm::io::commit(&false);
        }
    }
    println!("cycle-tracker-report-end: commit_public_inputs");
}
