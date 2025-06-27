//! ZisK guest program

#![no_main]

extern crate alloc;

use alloc::sync::Arc;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{Genesis, StatelessInput, validation::stateless_validation};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    println!("start read_input");
    let (input, genesis): (StatelessInput, Genesis) =
        bincode::deserialize(&ziskos::read_input()).unwrap();
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    println!("end read_input");

    println!("start validation");
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    println!("end validation");
}
