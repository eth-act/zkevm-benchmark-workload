//! OpenVM guest program

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::{chainconfig::ChainConfig, mpt::SparseState};
use openvm::io::read;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{StatelessInput, chain_spec::ChainSpec, stateless_validation_with_trie};

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read();
    let chain_config: ChainConfig = read();
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
