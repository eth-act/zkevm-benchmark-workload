//! Input types for the stateless validator guest program.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use anyhow::{Context, Result};
use ere_io_serde::{IoSerde, bincode};
use guest_libs::blobs::{BlockBody, BlockBodyEncoding, get_block_body};
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
    /// Experimental: Block body potentially compressed plus optional proof of equivalence.
    pub block_body: BlockBody,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BincodeBlockBody(
    #[serde_as(
        as = "reth_primitives_traits::serde_bincode_compat::BlockBody<reth_ethereum_primitives::TransactionSigned, alloy_consensus::Header>"
    )]
    pub alloy_consensus::BlockBody<reth_ethereum_primitives::TransactionSigned>,
);

impl Input {
    /// Create a new `Input` from the given `StatelessInput` and KZG commit mode.
    pub fn new(
        mut stateless_input: StatelessInput,
        block_body_encoding: BlockBodyEncoding,
        block_body_with_proof: bool,
    ) -> Result<Self> {
        let signers = recover_signers(stateless_input.block.body.transactions.iter())
            .map_err(|err| anyhow::anyhow!("recovering signers: {err}"))?;
        let body = BincodeBlockBody(core::mem::take(&mut stateless_input.block.body));

        let serialized = io_serde()
            .serialize(&body)
            .map_err(|e| anyhow::anyhow!("serializing block body: {e}"))?;
        let block_body = get_block_body(serialized, block_body_encoding, block_body_with_proof)
            .context("getting block body")?;

        Ok(Self {
            stateless_input,
            public_keys: signers,
            block_body,
        })
    }
}

/// Returns the serialization implementation for the stateless validator input.
pub fn io_serde() -> impl IoSerde {
    bincode::Bincode::legacy()
}
