//! OpenVM guest program
use std::sync::Arc;

use guest_libs::mpt::SparseState;
use openvm::io::read;
// For linker declarations:
use openvm_keccak256 as _;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{stateless_validation_with_trie, Genesis, StatelessInput};

openvm::init!();

/// Entry point.
pub fn main() {
    println!("start read_input");
    let input: StatelessInput = read();
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
