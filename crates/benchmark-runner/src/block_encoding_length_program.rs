//! Block encoding length calculation guest program.

use crate::{
    guest_programs::{GenericGuestIO, GuestIO},
    stateless_validator::read_benchmark_fixtures_folder,
};
use anyhow::*;
use block_encoding_length_guest::guest::{
    BlockEncodingFormat, BlockEncodingLengthGuest, BlockEncodingLengthInput,
};
use guest_libs::BincodeBlock;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::OnceLock};

/// Metadata for the block block length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEncodingLengthMetadata {
    format: String,
    block_hash: String,
    loop_count: u16,
}

/// Generate inputs for the block encoding lengths calculation guest programs.
pub fn block_encoding_length_inputs(
    input_folder: &Path,
    loop_count: u16,
    format: BlockEncodingFormat,
) -> Result<Vec<Box<dyn GuestIO>>> {
    read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let input = BlockEncodingLengthInput {
                block: BincodeBlock(bw.stateless_input.block.clone()),
                loop_count,
                format,
            };
            Ok(GenericGuestIO::<BlockEncodingLengthGuest, _> {
                name: bw.name,
                input,
                metadata: BlockEncodingLengthMetadata {
                    format: format!("{format:?}"),
                    block_hash: bw.stateless_input.block.hash_slow().to_string(),
                    loop_count,
                },
                output: OnceLock::from(()),
            }
            .into_boxed())
        })
        .collect()
}
