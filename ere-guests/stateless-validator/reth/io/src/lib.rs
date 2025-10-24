//! Input types for the stateless validator guest program.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use anyhow::Result;
use ere_io_serde::{IoSerde, bincode};
use guest_libs::senders::recover_signers;
use reth_stateless::{StatelessInput, UncompressedPublicKey};
use serde::{Deserialize, Serialize};

/// Input for the stateless validator guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The stateless input for the stateless validation function.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
}

impl Input {
    /// Create a new `Input` from the given `StatelessInput`.
    pub fn new(stateless_input: StatelessInput) -> Result<Self> {
        let signers = recover_signers(stateless_input.block.body.transactions.iter())
            .map_err(|err| anyhow::anyhow!("recovering signers: {err}"))?;
        Ok(Self {
            stateless_input,
            public_keys: signers,
        })
    }
}

/// Returns the serialization implementation for the stateless validator input.
pub fn io_serde() -> impl IoSerde {
    bincode::Bincode::legacy()
}
