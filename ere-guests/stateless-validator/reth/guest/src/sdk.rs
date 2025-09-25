//! SDK trait for stateless validator guest program.

use k256::ecdsa::VerifyingKey;
use reth_stateless::StatelessInput;

/// Trait that abstracts the SDK functions for reading inputs and committing outputs.
pub trait SDK {
    /// Reads the expected inputs for the block validation.
    fn read_inputs() -> (StatelessInput, Vec<VerifyingKey>);
    /// Commits the outputs from the block validation.
    fn commit_outputs(block_hash: [u8; 32], parent_hash: [u8; 32], is_valid: bool);
    /// Prints a message to the host environment.
    fn cycle_scope(scope: ScopeMarker, message: &str);
}

/// Enum to represent the start and end of a cycle scope for tracking purposes.
#[derive(Debug)]
pub enum ScopeMarker {
    /// Start of a cycle scope.
    Start,
    /// End of a cycle scope.
    End,
}
