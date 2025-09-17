//! Pico guest program

#![no_main]

extern crate alloc;
use std::error::Error;

use alloc::sync::Arc;

use alloy_primitives::FixedBytes;
use k256::ecdsa::VerifyingKey;
use pico_sdk::io::{commit, read_as};
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{ExecutionWitness, Genesis, StatelessInput, stateless_validation_with_trie};
use sparsestate::SparseState;

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("cycle-tracker-start: read_input");
    let input: StatelessInput = read_as();
    let public_keys: Vec<VerifyingKey> = read_as();
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

    let res = validate_block(
        input.block,
        input.witness,
        chain_spec,
        public_keys,
        evm_config,
    );

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
            println!("Block validation failed: {err}");
            commit(&header.hash_slow().0);
            commit(&parent_hash.0);
            commit(&false);
        }
    }
    println!("cycle-tracker-end: commit_public_inputs");
}

fn validate_block(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<VerifyingKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    println!("cycle-tracker-start: public_keys_validation");
    let recovered_block =
        guest_libs::senders::recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    println!("cycle-tracker-end: public_keys_validation");

    println!("cycle-tracker-start: validation");
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        witness,
        chain_spec,
        evm_config,
    )?;
    println!("cycle-tracker-end: validation");

    Ok(block_hash)
}
