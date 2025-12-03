//! [`Guest`] implementation for Reth stateless validator.

use alloc::{format, sync::Arc, vec::Vec};
use ere_io::{
    Io,
    serde::{IoSerde, bincode::BincodeLegacy},
};
use ere_platform_trait::Platform;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{
    Genesis, StatelessInput, UncompressedPublicKey, stateless_validation_with_trie,
};
use serde::{Deserialize, Serialize};
use sparsestate::SparseState;

pub use guest_libs::guest::Guest;

/// Input for the stateless validator guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RethStatelessValidatorInput {
    /// The stateless input for the stateless validation function.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
}

/// The public inputs are:
/// - block_hash : [u8;32]
/// - parent_hash : [u8;32]
/// - successful_block_validation : bool
pub type RethStatelessValidatorOutput = ([u8; 32], [u8; 32], bool);

/// [`Guest`] implementation for Reth stateless validator.
#[derive(Debug, Clone)]
pub struct RethStatelessValidatorGuest;

impl Guest for RethStatelessValidatorGuest {
    type Io = IoSerde<RethStatelessValidatorInput, RethStatelessValidatorOutput, BincodeLegacy>;

    fn compute<P: Platform>(input: <Self::Io as Io>::Input) -> <Self::Io as Io>::Output {
        let genesis = Genesis {
            config: input.stateless_input.chain_config.clone(),
            ..Default::default()
        };
        let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
        let evm_config = EthEvmConfig::new(chain_spec.clone());

        let (header, parent_hash) = P::cycle_scope("public_inputs_preparation", || {
            (
                input.stateless_input.block.header().clone(),
                input.stateless_input.block.parent_hash,
            )
        });

        let res = P::cycle_scope("validation", || {
            stateless_validation_with_trie::<SparseState, _, _>(
                input.stateless_input.block,
                input.public_keys,
                input.stateless_input.witness,
                chain_spec,
                evm_config,
            )
            .map(|(block_hash, _)| block_hash)
        });

        match res {
            Ok(block_hash) => (block_hash.0, parent_hash.0, true),
            Err(err) => {
                P::print(&format!("Block validation failed: {err}\n"));
                (header.hash_slow().0, parent_hash.0, false)
            }
        }
    }
}
