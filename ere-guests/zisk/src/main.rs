//! ZisK guest program

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{StatelessInput, fork_spec::ForkSpec, validation::stateless_validation};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let (input, fork_spec): (StatelessInput, ForkSpec) =
        bincode::deserialize(&ziskos::read_input()).unwrap();
    let chain_spec: Arc<ChainSpec> = Arc::new(fork_spec.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    println!("end validation");
}
