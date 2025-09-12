//! SP1 guest program

#![no_main]

extern crate alloc;
use std::error::Error;

use alloc::sync::Arc;

use alloy_primitives::FixedBytes;
use guest_libs::mpt::SparseState;
use k256::ecdsa::VerifyingKey;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{ExecutionWitness, Genesis, StatelessInput, stateless_validation_with_trie};

sp1_zkvm::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read::<StatelessInput>();
    let public_keys = sp1_zkvm::io::read::<Vec<VerifyingKey>>();

    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    println!("cycle-tracker-report-end: public_inputs_preparation");

    println!("cycle-tracker-report-start: commit_public_inputs");
    match validate_block(
        input.block,
        input.witness,
        chain_spec.clone(),
        public_keys,
        evm_config,
    ) {
        Ok(block_hash) => {
            sp1_zkvm::io::commit(&block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&true);
        }
        Err(err) => {
            println!("Block validation failed: {err}");
            sp1_zkvm::io::commit(&header.hash_slow().0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&false);
        }
    }
    println!("cycle-tracker-report-end: commit_public_inputs");
}

fn validate_block(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<VerifyingKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    println!("cycle-tracker-report-start: public_keys_validation");
    let recovered_block =
        guest_libs::senders::recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    println!("cycle-tracker-report-end: public_keys_validation");

    println!("cycle-tracker-report-start: validation");
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        witness,
        chain_spec,
        evm_config,
    )?;
    println!("cycle-tracker-report-end: validation");

    Ok(block_hash)
}
