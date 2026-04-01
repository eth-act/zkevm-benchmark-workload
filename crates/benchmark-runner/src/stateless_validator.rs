//! Stateless validator guest program.

use crate::guest_programs::{GenericGuestFixture, GuestFixture};
use anyhow::{Context, Result};
use ere_guests_guest::Guest;
use ere_guests_integration_tests::NoopPlatform;
use ere_guests_stateless_validator_ethrex::guest::{
    StatelessValidatorEthrexGuest, StatelessValidatorEthrexInput,
};
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
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

/// Lazily walks a fixture folder and yields each fixture file path.
pub fn iter_benchmark_fixture_paths(path: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
}

/// Reads and deserializes a single benchmark fixture file.
pub fn load_benchmark_fixture(path: &Path) -> Result<StatelessValidationFixture> {
    let content = std::fs::read(path)?;
    serde_json::from_slice(&content).with_context(|| format!("Failed to parse {}", path.display()))
}

/// Lazily prepares stateless validator inputs from a fixture folder.
pub fn stateless_validator_input_iter(
    input_folder: &Path,
    el: ExecutionClient,
    existing_output_dir: Option<&Path>,
) -> impl Iterator<Item = Result<Box<dyn GuestFixture>>> {
    stateless_validator_input_iter_from_paths(
        iter_benchmark_fixture_paths(input_folder),
        el,
        existing_output_dir.map(Path::to_path_buf),
    )
}

fn stateless_validator_input_iter_from_paths<I>(
    paths: I,
    el: ExecutionClient,
    existing_output_dir: Option<PathBuf>,
) -> impl Iterator<Item = Result<Box<dyn GuestFixture>>>
where
    I: Iterator<Item = PathBuf>,
{
    paths.filter_map(move |path| {
        match skip_existing_fixture_output(&path, existing_output_dir.as_deref()) {
            Ok(true) => None,
            Ok(false) => Some(
                load_benchmark_fixture(&path)
                    .and_then(|fixture| stateless_validator_input_from_fixture(fixture, el)),
            ),
            Err(err) => Some(Err(err)),
        }
    })
}

fn skip_existing_fixture_output(path: &Path, existing_output_dir: Option<&Path>) -> Result<bool> {
    let Some(existing_output_dir) = existing_output_dir else {
        return Ok(false);
    };

    let fixture_name = fixture_name_from_path(path)?;
    let output_path = existing_output_dir.join(format!("{fixture_name}.json"));
    if output_path.exists() {
        info!("Skipping {fixture_name} (already exists)");
        return Ok(true);
    }

    Ok(false)
}

fn fixture_name_from_path(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToOwned::to_owned)
        .with_context(|| format!("Failed to derive fixture name from {}", path.display()))
}

fn stateless_validator_input_from_fixture(
    fixture: StatelessValidationFixture,
    el: ExecutionClient,
) -> Result<Box<dyn GuestFixture>> {
    match el {
        ExecutionClient::Reth => reth_input_from_fixture(fixture),
        ExecutionClient::Ethrex => ethrex_input_from_fixture(fixture),
    }
}

fn ethrex_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    let input = StatelessValidatorEthrexInput::new(&stateless_input, success)
        .context("Failed to create Ethrex stateless validator input")?;
    let output = StatelessValidatorEthrexGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorEthrexGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}

fn reth_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    info!(
        "Preparing Reth stateless validator input for fixture {}",
        name
    );
    let input = StatelessValidatorRethInput::new(&stateless_input, success)
        .context("Failed to create Reth stateless validator input")?;

    let output = StatelessValidatorRethGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}
