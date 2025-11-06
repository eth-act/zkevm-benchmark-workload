//! SDK trait for stateless validator guest program.

use ere_platform_trait::Platform;

/// Trait that abstracts the SDK functions for reading inputs and committing outputs.
pub trait SDK {
    type Platform: Platform;

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
