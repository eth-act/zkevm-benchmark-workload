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
    /// Nethermind stateless block validation guest program.
    Nethermind,
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Gas used by the block
    pub block_used_gas: u64,
}

impl ExecutionClient {
    /// Returns the version string of the execution client (tag or short commit hash),
    /// extracted from the resolved `Cargo.lock` at build time. Nethermind is a
    /// temporary external binary path and does not come from `ere-guests`.
    pub const fn version(&self) -> &'static str {
        match self {
            Self::Reth => env!("RETH_EL_VERSION"),
            Self::Ethrex => env!("ETHREX_EL_VERSION"),
            Self::Nethermind => "external",
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
