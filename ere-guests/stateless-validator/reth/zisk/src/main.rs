//! ZisK guest program

#![no_main]

use std::{error::Error, sync::Arc};

use alloy_primitives::FixedBytes;
use k256::ecdsa::VerifyingKey;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{stateless_validation_with_trie, ExecutionWitness, Genesis, StatelessInput};
use sha2::{Digest, Sha256};
use sparsestate::SparseState;
use std::sync::Arc;

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input_bytes = ziskos::read_input();
    let mut offset = 0;
    let input: StatelessInput = bincode::deserialize(&input_bytes).unwrap();
    offset = bincode::serialized_size(&input).unwrap() as usize;
    let public_keys: Vec<VerifyingKey> = bincode::deserialize(&input_bytes[offset..]).unwrap();
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("end read_input");

    println!("start public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    println!("end public_inputs_preparation");

    let res = validate_block(
        input.block,
        input.witness,
        chain_spec,
        public_keys,
        evm_config,
    );

    println!("start commit_public_inputs");
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    let public_inputs = match res {
        Ok(block_hash) => (block_hash.0, parent_hash.0, true),
        Err(err) => {
            println!("Block validation failed: {err}");
            (header.hash_slow().0, parent_hash.0, false)
        }
    };
    let public_inputs_hash = Sha256::digest(bincode::serialize(&public_inputs).unwrap());
    public_inputs_hash
        .chunks_exact(4)
        .enumerate()
        .for_each(|(idx, bytes)| {
            ziskos::set_output(idx, u32::from_le_bytes(bytes.try_into().unwrap()))
        });
    println!("end commit_public_inputs");
}

fn validate_block(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<VerifyingKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    println!("start public_keys_validation");
    let recovered_block =
        guest_libs::senders::recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    println!("end public_keys_validation");

    println!("start validation");
    let block_hash = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        witness,
        chain_spec,
        evm_config,
    )?;
    println!("end validation");

    Ok(block_hash)
}
