//! Input types for the stateless validator guest program.

use anyhow::Result;
use ere_io_serde::{IoSerde, bincode};
use guest_libs::{
    io::{ProgramInput, ProgramInputError},
    senders::recover_signers,
};
use reth_stateless::{StatelessInput, UncompressedPublicKey};
use serde::Serialize;

/// Input for the stateless validator guest program.
#[derive(Debug, Clone, Serialize)]
pub struct Input {
    /// The stateless input for the stateless validation function.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub signers: Vec<UncompressedPublicKey>,
}

impl Input {
    /// Create a new `Input` from the given `StatelessInput`.
    pub fn new(stateless_input: StatelessInput) -> Result<Self> {
        let signers = recover_signers(stateless_input.block.body.transactions.iter())
            .map_err(|err| anyhow::anyhow!("recovering signers: {err}"))?;
        Ok(Self {
            stateless_input,
            signers,
        })
    }
}

// TODO: Auto-impl?
impl ProgramInput for Input {
    fn serialize_inputs(&self) -> Result<Vec<u8>, ProgramInputError> {
        bincode::Bincode::legacy()
            .serialize(self)
            .map_err(ProgramInputError::SerializationError)
    }
}
