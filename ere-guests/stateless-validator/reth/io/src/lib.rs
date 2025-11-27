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

/// Block body KZG commit options.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum BlockBodyKzgCommit {
    /// Disable KZG commitment calculation.
    #[default]
    None,
    /// Enable KZG with raw body encoding.
    Raw,
    /// Enable KZG with Snappy-compressed body encoding.
    Snappy,
}

/// Input for the stateless validator guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The stateless input for the stateless validation function.
    /// Experimental: does not contain block body.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
    /// Experimental: contains block body in some shape/form.
    pub block_body_bytes: BlockBodyBytes,
    /// Whether to compute KZG commitments.
    pub kzg_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockBodyBytes {
    Raw(Vec<u8>),
    CompressedSnappy(Vec<u8>),
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
        block_body_kzg_commit: BlockBodyKzgCommit,
    ) -> Result<Self> {
        let signers = recover_signers(stateless_input.block.body.transactions.iter())
            .map_err(|err| anyhow::anyhow!("recovering signers: {err}"))?;
        let body = BincodeBlockBody(core::mem::take(&mut stateless_input.block.body));
        let serialized = io_serde()
            .serialize(&body)
            .map_err(|e| anyhow::anyhow!("serializing block body: {e}"))?;

        let (block_body_bytes, kzg_enabled) = match block_body_kzg_commit {
            BlockBodyKzgCommit::None => (BlockBodyBytes::Raw(serialized), false),
            BlockBodyKzgCommit::Raw => (BlockBodyBytes::Raw(serialized), true),
            BlockBodyKzgCommit::Snappy => {
                let compressed = snap::raw::Encoder::new()
                    .compress_vec(&serialized)
                    .map_err(|e| anyhow::anyhow!("compressing block body with snappy: {e}"))?;
                (BlockBodyBytes::CompressedSnappy(compressed), true)
            }
        };

        Ok(Self {
            stateless_input,
            public_keys: signers,
            block_body_bytes,
            kzg_enabled,
        })
    }
}

/// Returns the serialization implementation for the stateless validator input.
pub fn io_serde() -> impl IoSerde {
    bincode::Bincode::legacy()
}
