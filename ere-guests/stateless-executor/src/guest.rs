//! [`Guest`] implementation for Reth stateless executor.

use alloc::{sync::Arc, vec::Vec};
use ere_io::{
    serde::{bincode::BincodeLegacy, IoSerde},
    Io,
};
use ere_platform_trait::Platform;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{Genesis, StatelessInput, UncompressedPublicKey};
use serde::{Deserialize, Serialize};
use sparsestate::SparseState;

pub use guest_libs::guest::Guest;

use crate::stateless_execution_with_trie;

/// Input for the stateless executor guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RethStatelessExecutorInput {
    /// The stateless input for the execution function.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
}

/// The public output is just a success/failure boolean.
pub type RethStatelessExecutorOutput = bool;

/// [`Guest`] implementation for Reth stateless executor.
///
/// This guest executes EVM transactions without validation checks,
/// useful for benchmarking pure EVM execution cycles.
#[derive(Debug, Clone)]
pub struct RethStatelessExecutorGuest;

impl Guest for RethStatelessExecutorGuest {
    type Io = IoSerde<RethStatelessExecutorInput, RethStatelessExecutorOutput, BincodeLegacy>;

    fn compute<P: Platform>(input: <Self::Io as Io>::Input) -> <Self::Io as Io>::Output {
        let genesis = Genesis {
            config: input.stateless_input.chain_config.clone(),
            ..Default::default()
        };
        let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
        let evm_config = EthEvmConfig::new(chain_spec.clone());

        P::cycle_scope("execution", || {
            stateless_execution_with_trie::<SparseState, _, _>(
                input.stateless_input.block,
                input.public_keys,
                input.stateless_input.witness,
                chain_spec,
                evm_config,
            )
        })
    }
}

