//! Risc0 guest program

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::{chainconfig::ChainConfig, mpt::SparseState};
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{chain_spec::ChainSpec, stateless_validation_with_trie, StatelessInput};
use risc0_zkvm::guest::env;

/// Entry point.
pub fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let input = env::read::<StatelessInput>();
    let chain_config = env::read::<ChainConfig>();
    let chain_spec: Arc<ChainSpec> = Arc::new(chain_config.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    let end = env::cycle_count();
    eprintln!("reading input (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
    stateless_validation_with_trie::<SparseState, _, _>(
        input.block,
        input.witness,
        chain_spec,
        evm_config,
    )
    .unwrap();
    let end = env::cycle_count();
    eprintln!("stateless validation (cycle tracker): {}", end - start);
}
