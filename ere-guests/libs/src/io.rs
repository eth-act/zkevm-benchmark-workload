use ere_io_serde::bincode;
use thiserror::Error;

/// Interface for serializing inputs to guest programs.
pub trait ProgramInput: Send + Sync {
    /// Serializes the program inputs into a byte vector.
    fn serialize_inputs(&self) -> Result<Vec<u8>, ProgramInputError>;
}

/// Interface for deserializing outputs from guest programs.
#[derive(Debug, Error)]
pub enum ProgramInputError {
    /// Error during serialization of inputs.
    #[error("Serialization error: {0}")]
    SerializationError(bincode::BincodeError),
}
