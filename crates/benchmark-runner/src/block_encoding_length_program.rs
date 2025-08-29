//! Block encoding length calculation guest program.

use std::path::Path;

use anyhow::*;
use ere_dockerized::ErezkVM;
use guest_libs::BincodeBlock;
use serde::{Deserialize, Serialize};
use zkvm_interface::Input;

use crate::{
    guest_programs::{GuestIO, GuestMetadata, OutputVerifier},
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

/// The encoding format used for the block encoding length calculation.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum BlockEncodingFormat {
    /// RLP encoding format
    Rlp,
    /// SSZ encoding format
    Ssz,
}

/// Generate inputs for the block encoding lengths calculation guest programs.
pub fn block_encoding_length_inputs(
    input_folder: &Path,
    loop_count: u16,
    format: BlockEncodingFormat,
) -> anyhow::Result<Vec<GuestIO<BlockEncodingLengthMetadata, ProgramOutputVerifier>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let mut stdin = Input::new();
            let metadata = BlockEncodingLengthMetadata {
                format: format!("{format:?}"),
                block_hash: bw.block_and_witness.block.hash_slow().to_string(),
                loop_count,
            };
            stdin.write(BincodeBlock(bw.block_and_witness.block));
            stdin.write(loop_count);
            stdin.write(format as u8);
            GuestIO {
                name: bw.name,
                input: stdin,
                metadata,
                output: ProgramOutputVerifier,
            }
        })
        .collect();

    Ok(guest_inputs)
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier;

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, _zkvm: ErezkVM, bytes: &[u8]) -> Result<bool> {
        Ok(bytes.is_empty())
    }
}
