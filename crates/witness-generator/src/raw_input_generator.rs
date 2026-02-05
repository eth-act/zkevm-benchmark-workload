//! Generate fixtures from pre-collected raw stateless input files.
//!
//! Reads a directory containing JSON-RPC response files (`eth_block.json` and
//! `debug_executionWitness.json`) organized in per-fixture subdirectories, along with a
//! shared `chain_config.json` at the root level.
//!
//! # Directory structure
//!
//! ```text
//! input_folder/
//! ├── chain_config.json
//! ├── fixture_1/
//! │   ├── eth_block.json
//! │   └── debug_executionWitness.json
//! └── fixture_2/
//!     ├── eth_block.json
//!     └── debug_executionWitness.json
//! ```

use crate::{Fixture, FixtureGenerator, Result, StatelessValidationFixture, WGError};
use alloy_genesis::ChainConfig;
use alloy_rpc_types_eth::Block;
use async_trait::async_trait;
use reth_ethereum_primitives::TransactionSigned;
use reth_stateless::{ExecutionWitness, StatelessInput};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

/// Generic JSON-RPC response envelope for deserializing local files that contain
/// JSON-RPC formatted data (`{"jsonrpc":"2.0","id":...,"result":{...}}`).
#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    /// The deserialized result payload.
    result: T,
}

/// Builder for configuring a [`RawInputFixtureGenerator`].
#[derive(Debug, Clone, Default)]
pub struct RawInputFixtureGeneratorBuilder {
    input_folder: Option<PathBuf>,
    skip_incomplete_fixtures: bool,
}

impl RawInputFixtureGeneratorBuilder {
    /// Sets the input folder containing `chain_config.json` and fixture subdirectories.
    ///
    /// # Errors
    ///
    /// Returns an error if the path does not exist or is not a directory.
    pub fn with_input_folder(mut self, path: PathBuf) -> Result<Self> {
        if !path.exists() {
            return Err(WGError::RawInputPathNotFound(path.display().to_string()));
        }
        if !path.is_dir() {
            return Err(WGError::RawInputPathNotDirectory(
                path.display().to_string(),
            ));
        }
        self.input_folder = Some(path.canonicalize()?);
        Ok(self)
    }

    /// When enabled, fixture subdirectories that are missing required files
    /// (`eth_block.json` or `debug_executionWitness.json`) will be skipped with a
    /// warning instead of causing the entire generation to fail.
    pub const fn with_skip_incomplete_fixtures(mut self, skip: bool) -> Self {
        self.skip_incomplete_fixtures = skip;
        self
    }

    /// Builds the configured [`RawInputFixtureGenerator`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input folder was not set.
    pub fn build(self) -> Result<RawInputFixtureGenerator> {
        let input_folder = self.input_folder.ok_or(WGError::RawInputPathNotSet)?;
        Ok(RawInputFixtureGenerator {
            input_folder,
            skip_incomplete_fixtures: self.skip_incomplete_fixtures,
        })
    }
}

/// Fixture generator that reads raw stateless input files from a local directory.
///
/// Expects `chain_config.json` at the root and one subdirectory per fixture, each
/// containing `eth_block.json` and `debug_executionWitness.json` in JSON-RPC response
/// format.
#[derive(Debug, Clone)]
pub struct RawInputFixtureGenerator {
    input_folder: PathBuf,
    skip_incomplete_fixtures: bool,
}

impl RawInputFixtureGenerator {
    /// Reads and deserializes a JSON file, returning a descriptive error on failure.
    fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
        let contents =
            std::fs::read_to_string(path).map_err(|e| WGError::RawInputFileReadError {
                path: path.display().to_string(),
                source: e,
            })?;
        serde_json::from_str(&contents).map_err(|e| WGError::RawInputDeserializationError {
            path: path.display().to_string(),
            source: e,
        })
    }
}

#[async_trait]
impl FixtureGenerator for RawInputFixtureGenerator {
    async fn generate(&self) -> Result<Vec<Box<dyn Fixture>>> {
        // Read the shared chain configuration.
        // Deserialize via the serde_bincode_compat wrapper which expects snake_case
        // field names (matching the format produced by StatelessInput serialization),
        // then convert into the canonical ChainConfig.
        let chain_config_path = self.input_folder.join("chain_config.json");
        let compat: alloy_genesis::serde_bincode_compat::ChainConfig<'_> =
            Self::read_json(&chain_config_path)?;
        let chain_config: ChainConfig = compat.into();

        // Collect and sort subdirectories for deterministic ordering
        let mut entries: Vec<_> = std::fs::read_dir(&self.input_folder)?
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_dir())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        let mut fixtures: Vec<Box<dyn Fixture>> = Vec::with_capacity(entries.len());

        for entry in entries {
            let fixture_dir = entry.path();
            let fixture_name = entry.file_name().to_string_lossy().to_string();

            // Validate that required files exist before attempting deserialization
            let block_path = fixture_dir.join("eth_block.json");
            if !block_path.exists() {
                if self.skip_incomplete_fixtures {
                    warn!("Skipping fixture '{fixture_name}': missing 'eth_block.json'");
                    continue;
                }
                return Err(WGError::RawInputMissingFile {
                    file: "eth_block.json".to_string(),
                    dir: fixture_dir.display().to_string(),
                });
            }
            let witness_path = fixture_dir.join("debug_executionWitness.json");
            if !witness_path.exists() {
                if self.skip_incomplete_fixtures {
                    warn!(
                        "Skipping fixture '{fixture_name}': missing 'debug_executionWitness.json'"
                    );
                    continue;
                }
                return Err(WGError::RawInputMissingFile {
                    file: "debug_executionWitness.json".to_string(),
                    dir: fixture_dir.display().to_string(),
                });
            }

            // Read eth_block.json and strip JSON-RPC envelope
            let block_rpc: JsonRpcResponse<Block<TransactionSigned>> =
                Self::read_json(&block_path)?;

            // Read debug_executionWitness.json and strip JSON-RPC envelope
            let witness_rpc: JsonRpcResponse<ExecutionWitness> = Self::read_json(&witness_path)?;

            let stateless_input = StatelessInput {
                block: block_rpc.result.into_consensus(),
                witness: witness_rpc.result,
                chain_config: chain_config.clone(),
            };

            info!("Loaded raw input fixture: {fixture_name}");

            fixtures.push(Box::new(StatelessValidationFixture {
                name: fixture_name,
                stateless_input,
                success: true,
            }));
        }

        Ok(fixtures)
    }
}
