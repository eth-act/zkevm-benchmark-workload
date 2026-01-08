//! Block encoding length calculation guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::read_benchmark_fixtures_folder,
};
use anyhow::{Context, Result};
use ere_guests_block_encoding_length::guest::{
    BlockEncodingFormat, BlockEncodingLengthGuest, BlockEncodingLengthInput,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
) -> Result<Vec<Box<dyn GuestFixture>>> {
    read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let input =
                BlockEncodingLengthInput::new(&bw.stateless_input.block, loop_count, format)
                    .context("Failed to create block encoding length input")?;
            let fixture = GenericGuestFixture::new::<BlockEncodingLengthGuest>(
                bw.name,
                input,
                (),
                BlockEncodingLengthMetadata {
                    format: format!("{format:?}"),
                    block_hash: bw.stateless_input.block.hash_slow().to_string(),
                    loop_count,
                },
            )?;
            Ok(fixture.into_boxed())
        })
        .collect()
}
