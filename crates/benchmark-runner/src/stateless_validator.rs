//! Stateless validator guest program.

use crate::guest_programs::GuestFixture;
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::path::Path;
use strum::{AsRefStr, EnumString};
use walkdir::WalkDir;
use witness_generator::StatelessValidationFixture;

pub mod ethrex;
pub mod reth;

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Gas used in the block.
    pub block_used_gas: u64,
}

/// Prepares the inputs for the stateless validator guest program based on the mode.
pub fn stateless_validator_inputs(
    input_folder: &Path,
    el: ExecutionClient,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    match el {
        ExecutionClient::Reth => reth::stateless_validator_inputs(input_folder),
        ExecutionClient::Ethrex => ethrex::stateless_validator_inputs(input_folder),
    }
}

/// Reads the benchmark fixtures folder and returns a list of block and witness pairs.
pub fn read_benchmark_fixtures_folder(path: &Path) -> Result<Vec<StatelessValidationFixture>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let bw: StatelessValidationFixture =
                    serde_json::from_slice(&content).map_err(|e| {
                        anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                    })?;
                Ok(bw)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect()
}
