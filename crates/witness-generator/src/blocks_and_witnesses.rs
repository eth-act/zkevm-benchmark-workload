use std::{fs, io, path::Path};

use alloy_rpc_types_trace::geth::DefaultFrame;
use anyhow::Result;
use async_trait::async_trait;
use reth_stateless::{StatelessInput, fork_spec::ForkSpec};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a collection of EVM execution traces for a block.
pub type BlockExecutionTraces = Vec<DefaultFrame>;

/// Represents a named collection of block/witness pairs for a specific Ethereum test case.
///
/// This structure typically corresponds to a single blockchain test case from the
/// `ethereum/tests` fixtures (however we are using `zkevm-fixtures`)
///  containing all the sequential block transitions within that test.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkFixture {
    /// Name of the block test case (e.g., "`ModExpAttackContract`").
    pub name: String,
    /// The required input for stateless validation.
    pub stateless_input: StatelessInput,
    /// The network fork specification (e.g., Shanghai, Cancun, Prague) active for this test case.
    // TODO: Don't think we want to pass this through maybe ForkSpec
    // TODO: Also Genesis file is wrong in chainspec
    // TODO: We can keep this initially and don't measure the time it takes to deserialize
    pub network: ForkSpec,
    /// The EVM execution trace
    pub evm_traces: Option<BlockExecutionTraces>,
}

/// Errors that can occur during serialization or deserialization of `BenchmarkFixture`.
#[derive(Error, Debug)]
pub enum BwError {
    /// Serde JSON (de)serialization error.
    #[error("serde JSON (de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Error during file system I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl BenchmarkFixture {
    /// Serializes a list of `BenchmarkFixture` test cases to a JSON pretty-printed string.
    ///
    /// # Errors
    ///
    /// Returns `BwError::Serde` if JSON serialization fails.
    pub fn to_json(items: &[Self]) -> Result<String, BwError> {
        serde_json::to_string_pretty(items).map_err(BwError::from)
    }

    /// Deserializes a list of `BenchmarkFixture` test cases from a JSON string.
    ///
    /// Assumes the input JSON was produced by [`Self::to_json`].
    ///
    /// # Errors
    ///
    /// Returns `BwError::Serde` if JSON deserialization fails.
    pub fn from_json(json: &str) -> Result<Vec<Self>, BwError> {
        serde_json::from_str(json).map_err(BwError::from)
    }

    /// Serializes `items` to pretty-printed JSON and writes them to `path`.
    ///
    /// The file is created if it does not exist and truncated if it does.
    /// Parent directories are *not* created automatically.
    ///
    /// # Errors
    ///
    /// Returns `BwError::Io` if any filesystem operation fails.
    /// Returns `BwError::Serde` if JSON serialization fails.
    pub fn to_path<P: AsRef<Path>>(path: P, items: &[Self]) -> Result<(), BwError> {
        let json = Self::to_json(items)?;
        fs::write(path, json).map_err(BwError::Io)?;
        Ok(())
    }

    /// Reads the file at `path` and deserializes a `Vec<BenchmarkFixture>` from its JSON content.
    ///
    /// Assumes the file contains JSON compatible with [`Self::from_json`].
    ///
    /// # Errors
    ///
    /// Returns `BwError::Io` if reading the file fails.
    /// Returns `BwError::Serde` if JSON deserialization fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, BwError> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(BwError::Io)?;
        Self::from_json(&contents)
    }
}

/// Trait for generating blocks and witnesses.
///
/// Implementors of this trait provide different strategies for generating
/// `BenchmarkFixture` collections, such as from test fixtures or RPC endpoints.
#[async_trait]
pub trait WitnessGenerator {
    /// Generates `BenchmarkFixture` fixtures.
    ///
    /// # Errors
    ///
    /// Returns an error if the generation process fails, including network issues,
    /// file I/O problems, or data processing errors.
    async fn generate(&self) -> Result<Vec<BenchmarkFixture>>;

    /// Generates `BenchmarkFixture` fixtures and writes them to the specified path.
    ///
    /// # Arguments
    /// * `path` - The directory path where fixture files will be written
    ///
    /// # Returns
    /// The number of fixture files successfully generated and written
    ///
    /// # Errors
    ///
    /// Returns an error if the generation fails or if writing to the path fails.
    async fn generate_to_path(&self, path: &Path) -> Result<usize>;
}
