//! Abstracted guest program

use std::{error::Error, sync::Arc};

use alloy_primitives::FixedBytes;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{
    ExecutionWitness, Genesis, UncompressedPublicKey, stateless_validation_with_trie,
};
use sparsestate::SparseState;

use crate::sdk::{PublicInputs, SDK, ScopeMarker};

/// Main entry point for the guest program.
pub fn ethereum_guest<S: SDK>() {
    S::cycle_scope(ScopeMarker::Start, "read_input");
    let (input, public_keys) = S::read_inputs();

    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    S::cycle_scope(ScopeMarker::End, "read_input");

    S::cycle_scope(ScopeMarker::Start, "public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    S::cycle_scope(ScopeMarker::End, "public_inputs_preparation");

    let res = validate_block::<S>(
        input.block,
        input.witness,
        chain_spec,
        public_keys,
        evm_config,
    );
    S::cycle_scope(ScopeMarker::Start, "commit_public_inputs");
    match res {
        Ok(block_hash) => {
            let public_inputs = PublicInputs {
                block_hash: block_hash.0,
                parent_hash: parent_hash.0,
                is_valid: true,
            };
            S::commit_outputs(&public_inputs);
        }
        Err(err) => {
            println!("Block validation failed: {err}");
            let public_inputs = PublicInputs {
                block_hash: header.hash_slow().0,
                parent_hash: parent_hash.0,
                is_valid: false,
            };
            S::commit_outputs(&public_inputs);
        }
    }
    S::cycle_scope(ScopeMarker::End, "commit_public_inputs");
}

fn validate_block<S: SDK>(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<UncompressedPublicKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    S::cycle_scope(ScopeMarker::Start, "validation");
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
    )?;
    S::cycle_scope(ScopeMarker::End, "validation");

    Ok(block_hash)
}
