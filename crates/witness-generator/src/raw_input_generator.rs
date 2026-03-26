//! Generate fixtures from pre-collected raw stateless input files.
//!
//! Expects a shared `chain_config.json` at the root level and a `raw_input_parts.txt`
//! file listing alternating `eth_block.json` and `debug_executionWitness.json` URLs.
//!
//! # Directory structure
//!
//! ```text
//! input_folder/
//! ├── chain_config.json
//! └── raw_input_parts.txt   # alternating eth_block.json / debug_executionWitness.json URLs
//! ```

use crate::{Fixture, FixtureGenerator, Result, StatelessValidationFixture, WGError};
use alloy_genesis::ChainConfig;
use alloy_rpc_types_eth::Block;
use async_trait::async_trait;
use reth_ethereum_primitives::TransactionSigned;
use serde::Deserialize;
use stateless::{ExecutionWitness, StatelessInput};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::task::JoinSet;
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
}

impl RawInputFixtureGeneratorBuilder {
    /// Sets the input folder containing `chain_config.json` and `raw_input_parts.txt`.
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

    /// Builds the configured [`RawInputFixtureGenerator`].
    ///
    /// # Errors
    ///
    /// Returns an error if the input folder was not set.
    pub fn build(self) -> Result<RawInputFixtureGenerator> {
        let input_folder = self.input_folder.ok_or(WGError::RawInputPathNotSet)?;
        Ok(RawInputFixtureGenerator { input_folder })
    }
}

/// Fixture generator that downloads raw stateless input files from URLs listed in
/// `raw_input_parts.txt`.
#[derive(Debug, Clone)]
pub struct RawInputFixtureGenerator {
    input_folder: PathBuf,
}

#[derive(Debug, Clone)]
struct FixtureUrlPair {
    fixture_name: String,
    block_url: String,
    witness_url: String,
}

#[derive(Debug)]
struct RawInputFixtureFailure {
    fixture_name: String,
    error: WGError,
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

    fn read_chain_config(&self) -> Result<ChainConfig> {
        let chain_config_path = self.input_folder.join("chain_config.json");
        let compat: alloy_genesis::serde_bincode_compat::ChainConfig<'_> =
            Self::read_json(&chain_config_path)?;
        Ok(compat.into())
    }

    fn read_url_pairs(&self) -> Result<Vec<FixtureUrlPair>> {
        let parts_path = self.input_folder.join("raw_input_parts.txt");
        let contents =
            std::fs::read_to_string(&parts_path).map_err(|e| WGError::RawInputFileReadError {
                path: parts_path.display().to_string(),
                source: e,
            })?;

        let urls: Vec<&str> = contents.lines().filter(|l| !l.trim().is_empty()).collect();
        if !urls.len().is_multiple_of(2) {
            return Err(WGError::RawInputInvalidUrlPair { line: urls.len() });
        }

        let mut pairs = Vec::with_capacity(urls.len() / 2);
        for (i, pair) in urls.chunks(2).enumerate() {
            let line = i * 2 + 1;
            if !pair[0].ends_with("eth_block.json") {
                return Err(WGError::RawInputInvalidUrlPair { line });
            }
            if !pair[1].ends_with("debug_executionWitness.json") {
                return Err(WGError::RawInputInvalidUrlPair { line: line + 1 });
            }

            pairs.push(FixtureUrlPair {
                fixture_name: Self::fixture_name_from_url(pair[0]),
                block_url: pair[0].to_string(),
                witness_url: pair[1].to_string(),
            });
        }

        Ok(pairs)
    }

    /// Extracts the fixture name from a URL path.
    ///
    /// Given a URL like:
    /// `https://host/results/runs/RUN_ID/TEST_NAME/post_test_rpc_calls/eth_block.json`
    /// returns `TEST_NAME`.
    fn fixture_name_from_url(url: &str) -> String {
        let segments: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();
        // segments: ["eth_block.json", "post_test_rpc_calls", "TEST_NAME", ...]
        if segments.len() >= 3 {
            segments[2].to_string()
        } else {
            url.to_string()
        }
    }

    /// Maximum number of concurrent fixture downloads.
    const DOWNLOAD_CONCURRENCY: usize = 4;

    /// Maximum number of retry attempts per download.
    const MAX_RETRIES: u32 = 15;

    fn retry_delay(attempt: u32) -> Duration {
        Duration::from_secs(u64::from(attempt))
    }

    /// Downloads a URL and deserializes the JSON-RPC response body, with retries.
    async fn download_json<T: serde::de::DeserializeOwned>(
        client: &reqwest::Client,
        url: &str,
    ) -> Result<T> {
        let mut last_err = None;
        for attempt in 1..=Self::MAX_RETRIES {
            match Self::download_json_once(client, url).await {
                Ok(val) => return Ok(val),
                Err(e) => {
                    if attempt < Self::MAX_RETRIES {
                        let delay = Self::retry_delay(attempt);
                        warn!(
                            "Download attempt {attempt}/{} failed for {url}: {e}, retrying in {delay:?}",
                            Self::MAX_RETRIES
                        );
                        tokio::time::sleep(delay).await;
                    }
                    last_err = Some(e);
                }
            }
        }
        Err(last_err.expect("at least one attempt"))
    }

    async fn download_json_once<T: serde::de::DeserializeOwned>(
        client: &reqwest::Client,
        url: &str,
    ) -> Result<T> {
        let bytes = client
            .get(url)
            .send()
            .await
            .map_err(|e| WGError::RawInputUrlDownloadError {
                url: url.to_string(),
                source: Box::new(e),
            })?
            .error_for_status()
            .map_err(|e| WGError::RawInputUrlDownloadError {
                url: url.to_string(),
                source: Box::new(e),
            })?
            .bytes()
            .await
            .map_err(|e| WGError::RawInputUrlDownloadError {
                url: url.to_string(),
                source: Box::new(e),
            })?;

        serde_json::from_slice(&bytes).map_err(|e| WGError::RawInputDeserializationError {
            path: url.to_string(),
            source: e,
        })
    }

    async fn download_fixture(
        client: &reqwest::Client,
        pair: FixtureUrlPair,
        chain_config: ChainConfig,
    ) -> Result<StatelessValidationFixture> {
        let (block_rpc, witness_rpc) = tokio::join!(
            Self::download_json::<JsonRpcResponse<Block<TransactionSigned>>>(
                client,
                &pair.block_url
            ),
            Self::download_json::<JsonRpcResponse<ExecutionWitness>>(client, &pair.witness_url),
        );

        let stateless_input = StatelessInput {
            block: block_rpc?.result.into_consensus(),
            witness: witness_rpc?.result,
            chain_config,
        };

        Ok(StatelessValidationFixture {
            name: pair.fixture_name,
            stateless_input,
            success: true,
        })
    }

    fn final_output_path(fixture_name: &str, path: &Path) -> PathBuf {
        path.join(format!("{fixture_name}.json"))
    }

    fn partial_output_path(fixture_name: &str, path: &Path) -> PathBuf {
        path.join(format!("{fixture_name}.json.part"))
    }

    fn write_fixture(fixture: &StatelessValidationFixture, path: &Path) -> Result<()> {
        let output_path = Self::final_output_path(&fixture.name, path);
        let part_path = Self::partial_output_path(&fixture.name, path);
        let buf =
            serde_json::to_vec_pretty(fixture).map_err(|e| WGError::FixtureSerializationError {
                name: fixture.name.clone(),
                source: e,
            })?;
        std::fs::write(&part_path, buf).map_err(|e| WGError::FixtureWriteError {
            path: part_path.display().to_string(),
            source: e,
        })?;
        std::fs::rename(&part_path, &output_path).map_err(|e| WGError::FixtureWriteError {
            path: output_path.display().to_string(),
            source: e,
        })
    }

    fn spawn_download_tasks(
        pairs: Vec<FixtureUrlPair>,
        client: Arc<reqwest::Client>,
        chain_config: ChainConfig,
    ) -> JoinSet<Result<StatelessValidationFixture>> {
        let total = pairs.len();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(Self::DOWNLOAD_CONCURRENCY));
        let mut tasks = JoinSet::new();

        for (i, pair) in pairs.into_iter().enumerate() {
            let client = Arc::clone(&client);
            let semaphore = Arc::clone(&semaphore);
            let chain_config = chain_config.clone();

            tasks.spawn(async move {
                let _permit = semaphore.acquire().await.expect("semaphore closed");
                info!(
                    "Downloading fixture {}/{}: {}",
                    i + 1,
                    total,
                    pair.fixture_name
                );
                Self::download_fixture(&client, pair, chain_config).await
            });
        }

        tasks
    }

    fn split_resume_pairs(
        pairs: Vec<FixtureUrlPair>,
        output_path: &Path,
    ) -> (Vec<FixtureUrlPair>, usize) {
        let mut pending = Vec::with_capacity(pairs.len());
        let mut skipped_existing = 0;

        for pair in pairs {
            if Self::final_output_path(&pair.fixture_name, output_path).exists() {
                skipped_existing += 1;
            } else {
                pending.push(pair);
            }
        }

        (pending, skipped_existing)
    }

    fn batch_failure_error(ready: usize, failures: &[RawInputFixtureFailure]) -> WGError {
        let details = failures
            .iter()
            .map(|failure| format!("{}: {}", failure.fixture_name, failure.error))
            .collect::<Vec<_>>()
            .join("\n");

        WGError::RawInputBatchFailed {
            ready,
            failed: failures.len(),
            details,
        }
    }
}

#[async_trait]
impl FixtureGenerator for RawInputFixtureGenerator {
    async fn generate(&self) -> Result<Vec<Box<dyn Fixture>>> {
        let chain_config = self.read_chain_config()?;
        let pairs = self.read_url_pairs()?;
        let total = pairs.len();
        let mut tasks =
            Self::spawn_download_tasks(pairs, Arc::new(reqwest::Client::new()), chain_config);

        let mut fixtures: Vec<Box<dyn Fixture>> = Vec::with_capacity(total);
        while let Some(result) = tasks.join_next().await {
            let fixture = result.map_err(|e| WGError::Other(Box::new(e)))??;
            info!("Loaded URL fixture: {}", fixture.name);
            fixtures.push(Box::new(fixture));
        }

        fixtures.sort_by(|a, b| a.name().cmp(b.name()));
        info!("Downloaded {}/{total} fixtures", fixtures.len());
        Ok(fixtures)
    }

    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let chain_config = self.read_chain_config()?;
        let pairs = self.read_url_pairs()?;
        let total = pairs.len();
        let (pending_pairs, skipped_existing) = Self::split_resume_pairs(pairs, path);
        let pending = pending_pairs.len();
        info!(
            "Raw input resume state: {skipped_existing}/{total} already present, {pending} pending"
        );

        let mut tasks = Self::spawn_download_tasks(
            pending_pairs,
            Arc::new(reqwest::Client::new()),
            chain_config,
        );

        let mut written = 0;
        let mut failures = Vec::new();
        while let Some(result) = tasks.join_next().await {
            let fixture = result.map_err(|e| WGError::Other(Box::new(e)))?;
            match fixture {
                Ok(fixture) => match Self::write_fixture(&fixture, path) {
                    Ok(()) => {
                        info!("Saved raw input fixture: {}", fixture.name);
                        written += 1;
                    }
                    Err(error) => {
                        warn!(
                            "Failed to write raw input fixture {}: {}",
                            fixture.name, error
                        );
                        failures.push(RawInputFixtureFailure {
                            fixture_name: fixture.name,
                            error,
                        });
                    }
                },
                Err(error) => {
                    let fixture_name = match &error {
                        WGError::RawInputUrlDownloadError { url, .. }
                        | WGError::RawInputDeserializationError { path: url, .. } => {
                            Self::fixture_name_from_url(url)
                        }
                        _ => "<unknown-fixture>".to_string(),
                    };
                    warn!("Failed raw input fixture {fixture_name}: {error}");
                    failures.push(RawInputFixtureFailure {
                        fixture_name,
                        error,
                    });
                }
            }
        }

        let ready = skipped_existing + written;
        info!(
            "Raw input run complete: {ready}/{total} ready, {written} written, {skipped_existing} skipped existing, {} failed",
            failures.len()
        );
        if failures.is_empty() {
            Ok(ready)
        } else {
            Err(Self::batch_failure_error(ready, &failures))
        }
    }
}
