//! Input definitions for the block encoding length calculation guest program.

use ere_io_serde::{IoSerde, bincode};
use guest_libs::BincodeBlock;
use serde::{Deserialize, Serialize};

/// Input for the block encoding length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The block to calculate the encoding length for.
    pub block: BincodeBlock,
    /// The number of times to repeat the encoding length calculation.
    pub loop_count: u16,
    /// The encoding format to use.
    pub format: BlockEncodingFormat,
}

/// Returns the serialization implementation for the block encoding length input.
pub fn io_serde() -> impl IoSerde {
    bincode::Bincode::legacy()
}

/// The encoding format used for the block encoding length calculation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum BlockEncodingFormat {
    /// RLP encoding format
    Rlp,
    /// SSZ encoding format
    Ssz,
}
