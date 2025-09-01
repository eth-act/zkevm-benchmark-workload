//! Guest program input generation and metadata types

use anyhow::Result;
use ere_dockerized::ErezkVM;
use serde::{de::DeserializeOwned, Serialize};
use zkvm_interface::Input;

/// Represents a guest program input with associated metadata
#[derive(Debug, Clone, Default)]
pub struct GuestIO<M: GuestMetadata, Output: OutputVerifier> {
    /// The name of the guest program input.
    pub name: String,
    /// The input to be provided to the guest program.
    pub input: Input,
    /// The expected output for the run.
    pub output: Output,
    /// Associated metadata for the guest program input.
    pub metadata: M,
}

/// Metadata trait for guest program inputs
pub trait GuestMetadata: Serialize + DeserializeOwned + Clone + Send + Sync {}

/// Verifies the output of a guest program
pub trait OutputVerifier: Clone + Send + Sync {
    /// Given a serialized output bytes from a zkVM, check if it matches the expected output
    fn check_serialized(&self, zkvm: ErezkVM, bytes: &[u8]) -> Result<bool>;
}
