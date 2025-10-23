//! SDK trait for stateless validator guest program.

use alloc::vec::Vec;

/// Trait that abstracts the SDK functions for reading inputs and committing outputs.
pub trait SDK {
    /// Reads input bytes.
    fn read_input() -> Vec<u8>;
    /// Commits the outputs from the block validation.
    fn commit_output(output: [u8; 32]);
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
