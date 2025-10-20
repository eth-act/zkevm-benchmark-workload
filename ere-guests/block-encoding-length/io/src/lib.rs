//! Input definitions for the block encoding length calculation guest program.

use ere_io_serde::{IoSerde, bincode};
use guest_libs::{
    BincodeBlock,
    io::{ProgramInput, ProgramInputError},
};
use serde::Serialize;

/// Input for the block encoding length calculation guest program.
#[derive(Debug, Clone, Serialize)]
pub struct Input {
    /// The block to calculate the encoding length for.
    pub block: BincodeBlock,
    /// The number of times to repeat the encoding length calculation.
    pub loop_count: u16,
    /// The encoding format to use.
    pub format: BlockEncodingFormat,
}

impl ProgramInput for Input {
    fn serialize_inputs(&self) -> Result<Vec<u8>, ProgramInputError> {
        bincode::Bincode::legacy()
            .serialize(self)
            .map_err(ProgramInputError::SerializationError)
    }
}

/// The encoding format used for the block encoding length calculation.
#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u8)]
pub enum BlockEncodingFormat {
    /// RLP encoding format
    Rlp,
    /// SSZ encoding format
    Ssz,
}
