//! Block encoding length calculation guest program.

use std::path::Path;

use anyhow::*;
use block_encoding_length_io::{BlockEncodingFormat, Input};
use ere_dockerized::ErezkVM;
use guest_libs::{io::ProgramInput, BincodeBlock};
use serde::{Deserialize, Serialize};

use crate::{
    guest_programs::{GuestIO, GuestMetadata, OutputVerifier, OutputVerifierResult},
    stateless_validator::read_benchmark_fixtures_folder,
};

/// Metadata for the block block length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEncodingLengthMetadata {
    format: String,
    block_hash: String,
    loop_count: u16,
}
impl GuestMetadata for BlockEncodingLengthMetadata {}

/// Generate inputs for the block encoding lengths calculation guest programs.
pub fn block_encoding_length_inputs(
    input_folder: &Path,
    loop_count: u16,
    format: BlockEncodingFormat,
) -> Result<Vec<GuestIO<BlockEncodingLengthMetadata, ProgramOutputVerifier>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            Ok(GuestIO {
                name: bw.name,
                input: Input {
                    block: BincodeBlock(bw.block_and_witness.block.clone()),
                    loop_count,
                    format,
                }
                .serialize_inputs()?,
                metadata: BlockEncodingLengthMetadata {
                    format: format!("{format:?}"),
                    block_hash: bw.block_and_witness.block.hash_slow().to_string(),
                    loop_count,
                },
                output: ProgramOutputVerifier,
            })
        })
        .collect::<Result<_, Error>>()?;

    Ok(guest_inputs)
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier;

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, _zkvm: ErezkVM, bytes: &[u8]) -> Result<OutputVerifierResult> {
        if bytes.is_empty() {
            return Ok(OutputVerifierResult::Match);
        }
        Ok(OutputVerifierResult::Mismatch(format!(
            "Expected empty output, got {bytes:?}"
        )))
    }
}
