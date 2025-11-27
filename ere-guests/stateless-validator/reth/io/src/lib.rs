//! Input types for the stateless validator guest program.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use anyhow::Result;
use ere_io_serde::{IoSerde, bincode};
use guest_libs::senders::recover_signers;
use reth_stateless::{StatelessInput, UncompressedPublicKey};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Input for the stateless validator guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The stateless input for the stateless validation function.
    /// Experimental: does not contain block body.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
    /// Experimental: contains block body in some shape/form.
    pub block_body: BlockBodyDA,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockBodyDA {
    Raw(
        #[serde_as(
            as = "reth_primitives_traits::serde_bincode_compat::BlockBody<reth_ethereum_primitives::TransactionSigned, alloy_consensus::Header>"
        )]
        alloy_consensus::BlockBody<reth_ethereum_primitives::TransactionSigned>,
    ),
    CompressedSnappy(Vec<u8>),
}

impl Input {
    /// Create a new `Input` from the given `StatelessInput`.
    pub fn new(mut stateless_input: StatelessInput) -> Result<Self> {
        let signers = recover_signers(stateless_input.block.body.transactions.iter())
            .map_err(|err| anyhow::anyhow!("recovering signers: {err}"))?;
        let body = core::mem::take(&mut stateless_input.block.body);
        Ok(Self {
            stateless_input,
            public_keys: signers,
            block_body: BlockBodyDA::Raw(body),
        })
    }
}

/// Returns the serialization implementation for the stateless validator input.
pub fn io_serde() -> impl IoSerde {
    bincode::Bincode::legacy()
}
