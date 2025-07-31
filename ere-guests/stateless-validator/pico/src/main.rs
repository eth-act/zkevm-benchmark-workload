//! Pico guest program

#![no_main]

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::{chainconfig::ChainConfig, mpt::SparseState};
use pico_sdk::io::read_as;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{
    StatelessInput, fork_spec::ForkSpec, stateless_validation_with_trie,
    validation::stateless_validation,
};

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read_as();
    let chain_config: ChainConfig = read_as();
    let chain_spec: Arc<ChainSpec> = Arc::new(chain_config.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("end read_input");

    println!("start validation");
    stateless_validation_with_trie::<SparseState, _, _>(
        input.block,
        input.witness,
        chain_spec,
        evm_config,
    )
    .unwrap();
    println!("end validation");
}
