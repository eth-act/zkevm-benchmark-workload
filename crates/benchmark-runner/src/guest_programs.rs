//! Guest program input generation and metadata types

use guest_libs::BincodeBlock;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;
use witness_generator::BlockAndWitness;
use zkvm_interface::Input;

/// Metadata trait for guest program inputs
pub trait GuestInputMetadata: Serialize + DeserializeOwned + Clone + Send + Sync {}

/// Represents a guest program input with associated metadata
#[derive(Debug, Clone)]
pub struct GuestInput<M: GuestInputMetadata> {
    /// The name of the guest program input.
    pub name: String,
    /// The standard input to be provided to the guest program.
    pub stdin: Input,
    /// Associated metadata for the guest program input.
    pub metadata: M,
}

// This implementation is required since the empty program does not have any metadata.
impl GuestInputMetadata for () {}

/// Generate inputs for the empty program guest program.
pub fn empty_program_inputs() -> GuestInput<()> {
    GuestInput {
        name: "empty_program".to_string(),
        stdin: Input::new(),
        metadata: (),
    }
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    block_used_gas: u64,
}
impl GuestInputMetadata for BlockMetadata {}

/// Generate inputs for the stateless validator guest program.
pub fn stateless_validator_inputs(
    input_folder: &Path,
) -> anyhow::Result<Vec<GuestInput<BlockMetadata>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let mut stdin = Input::new();
            stdin.write(bw.block_and_witness.clone());
            stdin.write(bw.chain_config);
            GuestInput {
                name: bw.name,
                stdin,
                metadata: BlockMetadata {
                    block_used_gas: bw.block_and_witness.block.gas_used,
                },
            }
        })
        .collect();

    Ok(guest_inputs)
}

/// Metadata for the block block length calculation guest program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEncodingLengthMetadata {
    format: String,
    block_hash: String,
    loop_count: u16,
}
impl GuestInputMetadata for BlockEncodingLengthMetadata {}

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
) -> anyhow::Result<Vec<GuestInput<BlockEncodingLengthMetadata>>> {
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
            GuestInput {
                name: bw.name,
                stdin,
                metadata,
            }
        })
        .collect();

    Ok(guest_inputs)
}

fn read_benchmark_fixtures_folder(path: &Path) -> anyhow::Result<Vec<BlockAndWitness>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let bw: BlockAndWitness = serde_json::from_slice(&content).map_err(|e| {
                    anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                })?;
                Ok(bw)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect::<anyhow::Result<Vec<BlockAndWitness>>>()
}
