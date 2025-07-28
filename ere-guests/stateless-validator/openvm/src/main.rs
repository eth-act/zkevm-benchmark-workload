//! OpenVM guest program

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use guest_libs::chainconfig::ChainConfig;
use openvm::io::read;

extern crate alloc;

use alloc::sync::Arc;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{StatelessInput, chain_spec::ChainSpec, validation::stateless_validation};

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read();
    let chain_config: ChainConfig = read();
    let chain_spec: Arc<ChainSpec> = Arc::new(chain_config.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    println!("end validation");
}
