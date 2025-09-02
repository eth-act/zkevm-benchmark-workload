//! Risc0 guest program

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::mpt::SparseState;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{stateless_validation_with_trie, Genesis, StatelessInput};
use risc0_zkvm::guest::env;

/// Entry point.
pub fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let input = env::read::<StatelessInput>();
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    let end = env::cycle_count();
    eprintln!("reading input (cycle tracker): {}", end - start);

    println!("public inputs preparation");
    let start = env::cycle_count();
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    let end = env::cycle_count();
    eprintln!("public inputs preparation (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
    let res = stateless_validation_with_trie::<SparseState, _, _>(
        input.block,
        input.witness,
        chain_spec,
        evm_config,
    );
    let end = env::cycle_count();
    eprintln!("stateless validation (cycle tracker): {}", end - start);

    println!("start commit public inputs");
    let start = env::cycle_count();
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    match res {
        Ok(block_hash) => {
            env::commit(&block_hash.0);
            env::commit(&parent_hash.0);
            env::commit(&true);
        }
        Err(_) => {
            env::commit(&header.hash_slow().0);
            env::commit(&parent_hash.0);
            env::commit(&false);
        }
    }
    let end = env::cycle_count();
    eprintln!("commit public inputs (cycle tracker): {}", end - start);
}
