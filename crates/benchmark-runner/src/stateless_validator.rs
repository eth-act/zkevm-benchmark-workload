//! Stateless validator guest program.

use crate::guest_programs::GenericGuestFixture;
use crate::guest_programs::GuestFixture;
use anyhow::Context;
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorOutput, StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use std::path::Path;
use witness_generator::StatelessValidationFixture;

use anyhow::Result;
use ere_guests_stateless_validator_ethrex::guest::{
    StatelessValidatorEthrexGuest, StatelessValidatorEthrexInput,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use walkdir::WalkDir;

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
    block_used_gas: u64,
}

/// Prepares the inputs for the stateless validator guest program based on the mode.
pub fn stateless_validator_inputs(
    input_folder: &Path,
    el: ExecutionClient,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    let fixtures = read_benchmark_fixtures_folder(input_folder)?;
    match el {
        ExecutionClient::Reth => reth_inputs_from_fixture(&fixtures),
        ExecutionClient::Ethrex => ethrex_inputs_from_fixture(&fixtures),
    }
}

/// Create a vector of `GuestFixture` instances from `StatelessValidationFixture`.
pub fn ethrex_inputs_from_fixture(
    fixture: &[StatelessValidationFixture],
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    fixture
        .iter()
        .map(|bw| {
            let input = StatelessValidatorEthrexInput::new(&bw.stateless_input)
                .context("Failed to create Ethrex stateless validator input")?;
            let output = StatelessValidatorOutput::new(
                bw.stateless_input.block.hash_slow(),
                bw.stateless_input.block.parent_hash,
                bw.success,
            );
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            let fixture =
                GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorEthrexGuest>(
                    bw.name.clone(),
                    input,
                    output,
                    metadata,
                )
                .output_sha256();

            Ok(fixture.into_boxed())
        })
        .collect()
}

/// Create a vector of `GuestFixture` instances from `StatelessValidationFixture`.
pub fn reth_inputs_from_fixture(
    fixture: &[StatelessValidationFixture],
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    fixture
        .iter()
        .map(|bw| {
            let input = StatelessValidatorRethInput::new(&bw.stateless_input)
                .context("Failed to create Reth stateless validator input")?;
            let output = StatelessValidatorOutput::new(
                bw.stateless_input.block.hash_slow(),
                bw.stateless_input.block.parent_hash,
                bw.success,
            );
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            let fixture = GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
                bw.name.clone(),
                input,
                output,
                metadata,
            )
            .output_sha256();

            Ok(fixture.into_boxed())
        })
        .collect()
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
