//! Block encoding length calculation guest program.

use std::path::Path;

use anyhow::*;
use block_encoding_length_io::{BlockEncodingFormat, Input};
use ere_dockerized::zkVMKind;
use ere_io_serde::IoSerde;
use guest_libs::BincodeBlock;
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
            let input = Input {
                block: BincodeBlock(bw.stateless_input.block.clone()),
                loop_count,
                format,
            };
            Ok(GuestIO {
                name: bw.name,
                input: block_encoding_length_io::io_serde()
                    .serialize(&input)
                    .map_err(|e| anyhow!("failed to serialize input: {e}"))?,
                metadata: BlockEncodingLengthMetadata {
                    format: format!("{format:?}"),
                    block_hash: bw.stateless_input.block.hash_slow().to_string(),
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
    fn check_serialized(&self, _zkvm: zkVMKind, bytes: &[u8]) -> Result<OutputVerifierResult> {
        if bytes.is_empty() {
            return Ok(OutputVerifierResult::Match);
        }
        Ok(OutputVerifierResult::Mismatch(format!(
            "Expected empty output, got {bytes:?}"
        )))
    }
}
