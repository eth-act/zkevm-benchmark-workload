//! Library for generating stateless validation fixtures for zkEVM benchmarking.
//!
//! Produces JSON fixtures containing Ethereum block data and execution witnesses from three
//! sources:
//!
//! - **EEST Generator** ([`eest_generator`]): Converts Ethereum Execution Spec Tests into fixtures
//! - **RPC Generator** ([`rpc_generator`]): Fetches blocks and witnesses from live Ethereum nodes
//! - **Raw Input Generator** ([`raw_input_generator`]): Reads pre-collected block and witness
//!   JSON-RPC response files from a local directory
//!
//! Core types: [`StatelessValidationFixture`] (block + witness), [`FixtureGenerator`].
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::{fs, path::Path};

use async_trait::async_trait;
use reth_stateless::StatelessInput;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod eest_generator;
pub mod raw_input_generator;
pub mod rpc_generator;

/// Error types for witness generation operations.
#[derive(Debug, Error)]
pub enum WGError {
    /// Error reading fixtures from file
    #[error("failed to read fixtures from file at {path}: {source}")]
    ReadFixtureError {
        /// Path to the fixture file
        path: String,
        /// Underlying I/O error
        source: std::io::Error,
    },

    /// EEST fixtures path does not exist
    #[error("EEST fixtures path '{0}' does not exist")]
    EestPathNotFound(String),

    /// EEST fixtures path is not a directory
    #[error("EEST fixtures path '{0}' is not a directory")]
    EestPathNotDirectory(String),

    /// Failed to download EEST fixtures
    #[error("failed to download EEST benchmark fixtures: {0}")]
    DownloadScriptFailed(String),

    /// Test suite path does not exist
    #[error("test suite path does not exist: {0}")]
    TestSuitePathNotFound(String),

    /// Failed to load test case
    #[error("failed to load test case from {path}: {source}")]
    TestCaseLoadError {
        /// Path to the test case file
        path: String,
        /// Underlying error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// No target block found for test case
    #[error("no target block found for test case {0}")]
    NoTargetBlock(String),

    /// Test case execution error
    #[error("test case execution error: {source}")]
    TestCaseExecutionError {
        /// Underlying error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Failed to serialize fixture
    #[error("failed to serialize fixture '{name}': {source}")]
    FixtureSerializationError {
        /// Name of the fixture
        name: String,
        /// Underlying serialization error
        source: serde_json::Error,
    },

    /// Failed to write fixture to path
    #[error("failed to write fixture to path '{path}': {source}")]
    FixtureWriteError {
        /// Path to write fixture to
        path: String,
        /// Underlying I/O error
        source: std::io::Error,
    },

    /// RPC error
    #[error("RPC error: {0}")]
    RpcError(String),

    /// Failed to fetch chain ID
    #[error("failed to fetch chain ID from RPC")]
    ChainIdFetchError,

    /// Unsupported chain
    #[error("unsupported chain ID: {0}")]
    UnsupportedChain(u64),

    /// Live polling not supported in generate method
    #[error("live polling is not supported in generate method. Use generate_to_path instead.")]
    LivePollingNotSupported,

    /// Failed to fetch latest block
    #[error("failed to fetch latest block")]
    LatestBlockFetchError,

    /// No block found for number
    #[error("no block found for number {0}")]
    BlockNotFoundForNumber(u64),

    /// No block found for hash
    #[error("no block found for hash {0}")]
    BlockNotFoundForHash(String),

    /// Cancellation token required
    #[error("cancellation token is required for live polling")]
    CancellationTokenRequired,

    /// Invalid header format
    #[error("invalid header format: '{header}'. Expected 'key:value'")]
    InvalidHeaderFormat {
        /// The invalid header string
        header: String,
    },

    /// Invalid header name
    #[error("invalid header name '{name}': {source}")]
    InvalidHeaderName {
        /// The invalid header name
        name: String,
        /// Underlying error
        source: http::header::InvalidHeaderName,
    },

    /// Invalid header value
    #[error("invalid header value '{value}': {source}")]
    InvalidHeaderValue {
        /// The invalid header value
        value: String,
        /// Underlying error
        source: http::header::InvalidHeaderValue,
    },

    /// Raw input path was not set in the builder
    #[error("raw input path was not set")]
    RawInputPathNotSet,

    /// Raw input path does not exist
    #[error("raw input path '{0}' does not exist")]
    RawInputPathNotFound(String),

    /// Raw input path is not a directory
    #[error("raw input path '{0}' is not a directory")]
    RawInputPathNotDirectory(String),

    /// Required file missing from a raw input fixture subdirectory
    #[error("missing required file '{file}' in fixture directory '{dir}'")]
    RawInputMissingFile {
        /// Name of the missing file
        file: String,
        /// Path to the fixture directory
        dir: String,
    },

    /// Failed to read a raw input file
    #[error("failed to read raw input file at {path}: {source}")]
    RawInputFileReadError {
        /// Path to the file
        path: String,
        /// Underlying I/O error
        source: std::io::Error,
    },

    /// Failed to deserialize a raw input file
    #[error("failed to deserialize raw input file at {path}: {source}")]
    RawInputDeserializationError {
        /// Path to the file
        path: String,
        /// Underlying deserialization error
        source: serde_json::Error,
    },

    /// Generic error for I/O, serialization, and other operations
    #[error("{0}")]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl From<std::io::Error> for WGError {
    fn from(err: std::io::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

impl From<serde_json::Error> for WGError {
    fn from(err: serde_json::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

/// Result type alias for witness generation operations.
pub type Result<T> = std::result::Result<T, WGError>;

/// Trait representing a fixture with serialization support and metadata access.
pub trait Fixture: erased_serde::Serialize + Send + Sync {
    /// Returns the unique name identifier for this fixture.
    fn name(&self) -> &str;
    /// Returns the block number associated with this fixture.
    fn block_number(&self) -> u64;
}

/// Trait for generating stateless validation fixtures.
#[async_trait]
pub trait FixtureGenerator: Sync {
    /// Generates a collection of fixtures based on the specified witness type.
    async fn generate(&self) -> Result<Vec<Box<dyn Fixture>>>;

    /// Generates fixtures and writes each to a JSON file in the specified directory.
    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let bws = self.generate().await?;
        for bw in &bws {
            let output_path = path.join(format!("{}.json", bw.name()));
            let mut buf = Vec::new();
            let mut serializer = serde_json::Serializer::pretty(&mut buf);
            erased_serde::serialize(bw.as_ref(), &mut serializer).map_err(|e| {
                WGError::FixtureSerializationError {
                    name: bw.name().to_owned(),
                    source: e,
                }
            })?;

            std::fs::write(&output_path, buf).map_err(|e| WGError::FixtureWriteError {
                path: output_path.display().to_string(),
                source: e,
            })?;
        }
        Ok(bws.len())
    }
}

/// A stateless validation fixture containing block data and witness information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatelessValidationFixture {
    /// Name of the blockchain test case (e.g., "`ModExpAttackContract`").
    pub name: String,
    /// The stateless input for the block validation.
    pub stateless_input: StatelessInput,
    /// Whether the stateless block validation is successful.
    pub success: bool,
}

impl StatelessValidationFixture {
    /// Serializes fixtures to a pretty-printed JSON string.
    pub fn to_json(items: &[Self]) -> Result<String> {
        Ok(serde_json::to_string_pretty(items)?)
    }

    /// Deserializes fixtures from a JSON string.
    pub fn from_json(json: &str) -> Result<Vec<Self>> {
        Ok(serde_json::from_str(json)?)
    }

    /// Serializes fixtures to JSON and writes to the specified file path.
    pub fn to_path<P: AsRef<Path>>(path: P, items: &[Self]) -> Result<()> {
        let json = Self::to_json(items)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Reads and deserializes fixtures from the specified file path.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|e| WGError::ReadFixtureError {
            path: path.display().to_string(),
            source: e,
        })?;
        Self::from_json(&contents)
    }

    /// Creates a new valid fixture from stateless input and a name.
    pub fn from_stateless_input(input: &StatelessInput, name: &str) -> Self {
        Self {
            name: name.to_string(),
            stateless_input: input.clone(),
            success: true,
        }
    }
}

impl Fixture for StatelessValidationFixture {
    fn name(&self) -> &str {
        &self.name
    }

    fn block_number(&self) -> u64 {
        self.stateless_input.block.number
    }
}
