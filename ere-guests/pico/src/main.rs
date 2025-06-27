#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::read_as;

extern crate alloc;

use alloc::sync::Arc;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{validation::stateless_validation, Genesis, StatelessInput};

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read_as();
    let genesis: Genesis = read_as();
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    println!("end validation");
}
