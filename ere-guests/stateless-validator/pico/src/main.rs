//! Pico guest program

#![no_main]

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::mpt::SparseState;
use pico_sdk::io::read_as;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{
    fork_spec::ForkSpec, stateless_validation_with_trie, validation::stateless_validation,
    StatelessInput,
};

pico_sdk::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read_as();
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
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
