//! ZisK guest program

#![no_main]

use guest_libs::mpt::SparseState;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{stateless_validation_with_trie, Genesis, StatelessInput};
use sha2::{Digest, Sha256};
use std::sync::Arc;

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = bincode::deserialize(&ziskos::read_input()).unwrap();
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

    println!("start validation");
    let recovered_block = guest_libs::senders::recover_block(input.block, &chain_spec).unwrap();
    let res = stateless_validation_with_trie::<SparseState, _, _>(
        recovered_block,
        input.witness,
        chain_spec,
        evm_config,
    );
    println!("end validation");

    println!("start commit_public_inputs");
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    let public_inputs = match res {
        Ok(block_hash) => (block_hash.0, parent_hash.0, true),
        Err(_) => (header.hash_slow().0, parent_hash.0, false),
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
