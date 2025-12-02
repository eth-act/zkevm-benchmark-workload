//! [`Guest`] implementation for the block encoding length calculation.

use ere_io::{
    Io,
    serde::{IoSerde, bincode::BincodeLegacy},
};
use ere_platform_trait::Platform;
use guest_libs::{BincodeBlock, block_ssz};
use reth_ethereum_primitives::Block;
use serde::{Deserialize, Serialize};
use ssz::Encode;

pub use guest_libs::guest::Guest;

/// The encoding format used for the block encoding length calculation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum BlockEncodingFormat {
    /// RLP encoding format
    Rlp,
    /// SSZ encoding format
    Ssz,
}

/// Input for the block encoding length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEncodingLengthInput {
    /// The block to calculate the encoding length for.
    pub block: BincodeBlock,
    /// The number of times to repeat the encoding length calculation.
    pub loop_count: u16,
    /// The encoding format to use.
    pub format: BlockEncodingFormat,
}

/// [`Guest`] implementation for the block encoding length calculation.
#[derive(Debug, Clone)]
pub struct BlockEncodingLengthGuest;

impl Guest for BlockEncodingLengthGuest {
    type Io = IoSerde<BlockEncodingLengthInput, (), BincodeLegacy>;

    fn compute<P: Platform>(input: <Self::Io as Io>::Input) -> <Self::Io as Io>::Output {
        match input.format {
            BlockEncodingFormat::Rlp => {
                P::cycle_scope("block_encoding_length_calculation", || {
                    for _ in 0..input.loop_count {
                        Block::rlp_length_for(&input.block.header, &input.block.body);
                    }
                });
            }
            BlockEncodingFormat::Ssz => {
                let block: block_ssz::Block =
                    P::cycle_scope("block_format_conversion", || input.block.0.into());

                P::cycle_scope("block_encoding_length_calculation", || {
                    for _ in 0..input.loop_count {
                        block.ssz_bytes_len();
                    }
                });
            }
        }
    }
}
