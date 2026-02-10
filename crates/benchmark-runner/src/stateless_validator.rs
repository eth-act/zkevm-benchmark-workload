//! Stateless validator guest program.

use crate::guest_programs::GenericGuestFixture;
use crate::guest_programs::GuestFixture;
use anyhow::{Context, Result};
use ere_guests_guest::Guest;
use ere_guests_integration_tests::NoopPlatform;
use ere_guests_stateless_validator_ethrex::guest::{
    StatelessValidatorEthrexGuest, StatelessValidatorEthrexInput,
};
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::path::Path;
use strum::{AsRefStr, EnumString};
use tracing::info;
use walkdir::WalkDir;
use witness_generator::StatelessValidationFixture;

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
    /// Gas used by the block
    pub block_used_gas: u64,
}

impl ExecutionClient {
    /// Returns the version string of the execution client (tag or short commit hash),
    /// extracted from the resolved `Cargo.lock` at build time.
    pub const fn version(&self) -> &'static str {
        match self {
            Self::Reth => env!("RETH_EL_VERSION"),
            Self::Ethrex => env!("ETHREX_EL_VERSION"),
        }
    }
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
    fixtures: &[StatelessValidationFixture],
) -> Result<Vec<Box<dyn GuestFixture>>> {
    fixtures
        .iter()
        .map(|bw| {
            let input = StatelessValidatorEthrexInput::new(&bw.stateless_input, bw.success)
                .context("Failed to create Ethrex stateless validator input")?;
            let output = StatelessValidatorEthrexGuest::compute::<NoopPlatform>(input.clone());
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            let fixture =
                GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorEthrexGuest>(
                    bw.name.clone(),
                    input,
                    output,
                    metadata,
                )?
                .output_sha256();

            Ok(fixture.into_boxed())
        })
        .collect()
}

/// Create a vector of `GuestFixture` instances from `StatelessValidationFixture`.
pub fn reth_inputs_from_fixture(
    fixtures: &[StatelessValidationFixture],
) -> Result<Vec<Box<dyn GuestFixture>>> {
    fixtures
        .par_iter()
        .map(|bw| {
            info!(
                "Preparing Reth stateless validator input for fixture {}",
                bw.name
            );
            let input = StatelessValidatorRethInput::new(&bw.stateless_input, bw.success)
                .context("Failed to create Reth stateless validator input")?;

            let output = StatelessValidatorRethGuest::compute::<NoopPlatform>(input.clone());
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            let fixture = GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
                bw.name.clone(),
                input,
                output,
                metadata,
            )?
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
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .par_bridge()
        .map(|entry| {
            let content = std::fs::read(entry.path())?;
            let fixture: StatelessValidationFixture = serde_json::from_slice(&content)
                .with_context(|| format!("Failed to parse {}", entry.path().display()))?;
            Ok(fixture)
        })
        .collect()
}
