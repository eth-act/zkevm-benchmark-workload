//! Stateless validator guest program.

mod eest;
mod fixtures;
mod inputs;

use crate::guest_programs::GuestFixture;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use strum::{AsRefStr, EnumString};

pub use fixtures::{benchmark_fixture_paths, iter_benchmark_fixture_paths, load_benchmark_fixture};

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
    /// Zilkworm stateless block validation guest program.
    Zilkworm,
    /// Zesu stateless block validation guest program.
    Zesu,
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Gas used by the block
    pub block_used_gas: u64,
}

/// Reth version used to build the Reth artifacts in ere-guests v0.13.0.
const RETH_EL_VERSION: &str = "v2.3.0";
/// Zesu version republished with the ere-guests v0.13.0 artifacts.
const ZESU_EL_VERSION: &str = "bal-devnet-7-2026-06-24";

impl ExecutionClient {
    /// Returns the version string associated with the selected guest artifact.
    pub const fn version(&self) -> &'static str {
        match self {
            Self::Reth => RETH_EL_VERSION,
            Self::Ethrex => ere_guests_stateless_validator_ethrex::EL_VERSION,
            Self::Zilkworm => env!("ZILKWORM_EL_VERSION"),
            Self::Zesu => ZESU_EL_VERSION,
        }
    }
}

/// Lazily prepares stateless validator inputs from a fixture folder.
pub fn stateless_validator_input_iter(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
    el: ExecutionClient,
    existing_output_dir: Option<&Path>,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>>> {
    fixtures::stateless_validator_input_iter(
        input_folder,
        selected_fixtures,
        el,
        existing_output_dir,
    )
}
