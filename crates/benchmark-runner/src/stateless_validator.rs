//! Stateless validator guest program.

use std::path::Path;

use alloy_primitives::FixedBytes;
use anyhow::Result;
use ere_dockerized::ErezkVM;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use witness_generator::BlockAndWitness;
use zkvm_interface::Input;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier};

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    block_used_gas: u64,
}
impl GuestMetadata for BlockMetadata {}

/// Generate inputs for the stateless validator guest program.
pub fn stateless_validator_inputs(
    input_folder: &Path,
) -> anyhow::Result<Vec<GuestIO<BlockMetadata, ProgramOutputVerifier>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let mut stdin = Input::new();
            stdin.write(bw.block_and_witness.clone());
            GuestIO {
                name: bw.name,
                input: stdin,
                metadata: BlockMetadata {
                    block_used_gas: bw.block_and_witness.block.gas_used,
                },
                output: ProgramOutputVerifier {
                    block_hash: bw.block_and_witness.block.hash_slow(),
                    parent_hash: bw.block_and_witness.block.parent_hash,
                    success: bw.success,
                },
            }
        })
        .collect();

    Ok(guest_inputs)
}

/// Reads the benchmark fixtures folder and returns a list of block and witness pairs.
pub fn read_benchmark_fixtures_folder(path: &Path) -> anyhow::Result<Vec<BlockAndWitness>> {
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

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier {
    block_hash: FixedBytes<32>,
    parent_hash: FixedBytes<32>,
    success: bool,
}

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, zkvm: ErezkVM, bytes: &[u8]) -> Result<bool> {
        match zkvm {
            ErezkVM::SP1 => {
                let mut bytes: &[u8] = bytes;
                let block_hash: FixedBytes<32> = zkvm.deserialize_from(&mut bytes)?;
                let parent_hash: FixedBytes<32> = zkvm.deserialize_from(&mut bytes)?;
                let success: bool = zkvm.deserialize_from(&mut bytes)?;

                if block_hash != self.block_hash {
                    anyhow::bail!(
                        "Block hash mismatch: expected {:?}, got {:?}",
                        self.block_hash,
                        block_hash
                    );
                }
                if parent_hash != self.parent_hash {
                    anyhow::bail!(
                        "Parent hash mismatch: expected {:?}, got {:?}",
                        self.parent_hash,
                        parent_hash
                    );
                }
                if success != self.success {
                    anyhow::bail!(
                        "Success mismatch: expected {:?}, got {:?}",
                        self.success,
                        success
                    );
                }

                Ok(true)
            }
            _ => unimplemented!(),
        }
    }
}
