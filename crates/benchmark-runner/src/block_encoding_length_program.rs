//! Block encoding length calculation guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::{benchmark_fixture_paths, load_benchmark_fixture},
};
use anyhow::{Context, Result};
use ere_guests_block_encoding_length::guest::{
    BlockEncodingFormat, BlockEncodingLengthGuest, BlockEncodingLengthInput,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Metadata for the block block length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEncodingLengthMetadata {
    format: String,
    block_hash: String,
    loop_count: u16,
}

/// Lazily generates inputs for the block encoding length guest program.
pub fn block_encoding_length_input_iter(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
    loop_count: u16,
    format: BlockEncodingFormat,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>>> {
    Ok(block_encoding_length_input_iter_from_paths(
        benchmark_fixture_paths(input_folder, selected_fixtures)?.into_iter(),
        loop_count,
        format,
    ))
}

fn block_encoding_length_input_iter_from_paths<I>(
    paths: I,
    loop_count: u16,
    format: BlockEncodingFormat,
) -> impl Iterator<Item = Result<Box<dyn GuestFixture>>>
where
    I: Iterator<Item = PathBuf>,
{
    paths.map(move |path| {
        let fixture = load_benchmark_fixture(&path)?;
        let witness_generator::StatelessValidationFixture {
            name,
            stateless_input,
            ..
        } = fixture;
        let input = BlockEncodingLengthInput::new(&stateless_input.block, loop_count, format)
            .context("Failed to create block encoding length input")?;
        let fixture = GenericGuestFixture::new::<BlockEncodingLengthGuest>(
            name,
            input,
            (),
            BlockEncodingLengthMetadata {
                format: format!("{format:?}"),
                block_hash: stateless_input.block.hash_slow().to_string(),
                loop_count,
            },
        )?;
        Ok(fixture.into_boxed())
    })
}
