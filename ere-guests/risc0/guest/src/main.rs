use risc0_zkvm::guest::env;

extern crate alloc;

use alloc::sync::Arc;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{fork_spec::ForkSpec, validation::stateless_validation, StatelessInput};

/// Entry point.
pub fn main() {
    println!("start reading input");
    let start = env::cycle_count();
    let input = env::read::<StatelessInput>();
    let fork_spec = env::read::<ForkSpec>();
    let chain_spec: Arc<ChainSpec> = Arc::new(fork_spec.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    let end = env::cycle_count();
    eprintln!("reading input (cycle tracker): {}", end - start);

    println!("start stateless validation");
    let start = env::cycle_count();
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    let end = env::cycle_count();
    eprintln!("stateless validation (cycle tracker): {}", end - start);
}
