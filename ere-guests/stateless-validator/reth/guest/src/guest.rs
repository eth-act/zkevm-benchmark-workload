//! Abstracted guest program

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::error::Error;

use alloy_primitives::FixedBytes;
use ere_io_serde::IoSerde;
use k256::sha2::{Digest, Sha256};
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_guest_io::{Input, io_serde};
use reth_primitives_traits::Block;
use reth_stateless::{
    ExecutionWitness, Genesis, UncompressedPublicKey, stateless_validation_with_trie,
};
use sparsestate::SparseState;

use crate::sdk::{SDK, ScopeMarker};

/// Main entry point for the guest program.
pub fn ethereum_guest<S: SDK>() {
    S::cycle_scope(ScopeMarker::Start, "read_input");
    let input: Input = io_serde()
        .deserialize(&S::read_input())
        .expect("Failed to read input");

    let genesis = Genesis {
        config: input.stateless_input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    S::cycle_scope(ScopeMarker::End, "read_input");

    S::cycle_scope(ScopeMarker::Start, "public_inputs_preparation");
    let header = input.stateless_input.block.header().clone();
    let parent_hash = input.stateless_input.block.parent_hash;
    S::cycle_scope(ScopeMarker::End, "public_inputs_preparation");

    let res = validate_block::<S>(
        input.stateless_input.block,
        input.stateless_input.witness,
        chain_spec,
        input.public_keys,
        evm_config,
    );
    S::cycle_scope(ScopeMarker::Start, "commit_public_inputs");
    match res {
        Ok(block_hash) => {
            let public_inputs = (block_hash.0, parent_hash.0, true);
            let public_inputs_hash: [u8; 32] = Sha256::digest(
                bincode_v2::serde::encode_to_vec(&public_inputs, bincode_v2::config::legacy())
                    .unwrap(),
            )
            .into();
            S::commit_output(public_inputs_hash);
        }
        Err(_err) => {
            #[cfg(feature = "std")]
            println!("Block validation failed: {_err}");
            let public_inputs = (header.hash_slow().0, parent_hash.0, false);
            let public_inputs_hash: [u8; 32] = Sha256::digest(
                bincode_v2::serde::encode_to_vec(&public_inputs, bincode_v2::config::legacy())
                    .unwrap(),
            )
            .into();
            S::commit_output(public_inputs_hash);
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
