//! Risc0 guest program

extern crate alloc;
use std::error::Error;

use alloc::sync::Arc;

use alloy_primitives::FixedBytes;
use k256::ecdsa::VerifyingKey;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{stateless_validation_with_trie, ExecutionWitness, Genesis, StatelessInput};
use risc0_zkvm::guest::env;
use sparsestate::SparseState;

/// Entry point.
pub fn main() {
    println!("start reading_input");
    let start = env::cycle_count();
    let input = env::read::<StatelessInput>();
    let public_keys = env::read::<Vec<VerifyingKey>>();

    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    let end = env::cycle_count();
    eprintln!("reading_input (cycle tracker): {}", end - start);

    println!("public_inputs_preparation");
    let start = env::cycle_count();
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    let end = env::cycle_count();
    eprintln!("public_inputs_preparation (cycle tracker): {}", end - start);

    let res = validate_block(
        input.block,
        input.witness,
        chain_spec.clone(),
        public_keys,
        evm_config,
    );
    println!("start commit_public_inputs");
    let start = env::cycle_count();
    match res {
        Ok(block_hash) => {
            env::commit(&block_hash.0);
            env::commit(&parent_hash.0);
            env::commit(&true);
        }
        Err(err) => {
            println!("Block validation failed: {err}");
            env::commit(&header.hash_slow().0);
            env::commit(&parent_hash.0);
            env::commit(&false);
        }
    }
    let end = env::cycle_count();
    eprintln!("commit_public_inputs (cycle tracker): {}", end - start);
}

fn validate_block(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<VerifyingKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    println!("start public_keys_validation");
    let start = env::cycle_count();
    let recovered_block =
        guest_libs::senders::recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    let end = env::cycle_count();
    eprintln!("public_keys_validation (cycle tracker): {}", end - start);

    println!("start stateless_validation");
    let start = env::cycle_count();
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        witness,
        chain_spec,
        evm_config,
    )?;
    let end = env::cycle_count();
    eprintln!("stateless_validation (cycle tracker): {}", end - start);

    Ok(block_hash)
}
