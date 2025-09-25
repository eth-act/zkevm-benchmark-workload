//! Abstracted guest program

use std::{error::Error, sync::Arc};

use alloy_primitives::FixedBytes;
use k256::ecdsa::VerifyingKey;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{ExecutionWitness, Genesis, stateless_validation_with_trie};
use sparsestate::SparseState;

use guest_libs::senders::recover_block_with_public_keys;

use crate::sdk::{CycleScope, SDK};

/// Main entry point for the guest program.
pub fn ethereum_guest<S: SDK>() {
    S::cycle_scope(CycleScope::Start, "read_input");
    let (input, public_keys) = S::read_inputs().unwrap();

    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    S::cycle_scope(CycleScope::End, "read_input");

    S::cycle_scope(CycleScope::Start, "public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    S::cycle_scope(CycleScope::End, "public_inputs_preparation");

    let res = validate_block::<S>(
        input.block,
        input.witness,
        chain_spec,
        public_keys,
        evm_config,
    );
    S::cycle_scope(CycleScope::Start, "commit_public_inputs");
    match res {
        Ok(block_hash) => {
            S::commit_outputs(block_hash.0, parent_hash.0, true);
        }
        Err(err) => {
            println!("Block validation failed: {err}");
            S::commit_outputs(header.hash_slow().0, parent_hash.0, false);
        }
    }
    S::cycle_scope(CycleScope::End, "commit_public_inputs");
}

fn validate_block<S: SDK>(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<VerifyingKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    S::cycle_scope(CycleScope::Start, "public_keys_validation");
    let recovered_block = recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    S::cycle_scope(CycleScope::End, "public_keys_validation");

    S::cycle_scope(CycleScope::Start, "validation");
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        witness,
        chain_spec,
        evm_config,
    )?;
    S::cycle_scope(CycleScope::End, "validation");

    Ok(block_hash)
}
