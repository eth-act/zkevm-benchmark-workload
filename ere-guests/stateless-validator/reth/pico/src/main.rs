//! Pico guest program

#![no_main]

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::mpt::SparseState;
use pico_sdk::io::{commit, read_as};
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{stateless_validation_with_trie, Genesis, StatelessInput};

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("cycle-tracker-start: read_input");
    let input: StatelessInput = read_as();
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("cycle-tracker-end: read_input");

    println!("cycle-tracker-start: public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    println!("cycle-tracker-end: public_inputs_preparation");

    println!("cycle-tracker-start: validation");
    let recovered_block = guest_libs::senders::recover_block(input.block, &chain_spec).unwrap();
    let res = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        input.witness,
        chain_spec,
        evm_config,
    );
    println!("cycle-tracker-end: validation");

    println!("cycle-tracker-start: commit_public_inputs");
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    match res {
        Ok(block_hash) => {
            commit(&block_hash.0);
            commit(&parent_hash.0);
            commit(&true);
        }
        Err(err) => {
            commit(&header.hash_slow().0);
            commit(&parent_hash.0);
            commit(&false);
        }
    }
    println!("cycle-tracker-end: commit_public_inputs");
}
