//! Generate block and witnesses from an RPC endpoint

use crate::{Fixture, FixtureGenerator, Result, StatelessValidationFixture, WGError};
use alloy_eips::BlockNumberOrTag;
use alloy_genesis::ChainConfig;
use alloy_rpc_types_eth::{Block, Header, Receipt, Transaction, TransactionRequest};
use async_trait::async_trait;
use http::{HeaderName, HeaderValue};
use jsonrpsee::{
    http_client::{HeaderMap, HttpClient, HttpClientBuilder},
    tracing::{error, info},
};
use reth_chainspec::{Chain, HOLESKY, HOODI, MAINNET, NamedChain, SEPOLIA};
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_api::{DebugApiClient, EthApiClient};
use reth_stateless::StatelessInput;
use std::{path::Path, str::FromStr};
use tokio_util::sync::CancellationToken;

/// Builder for configuring an RPC client that fetches blocks and witnesses.
#[derive(Debug, Clone, Default)]
pub struct RpcBlocksAndWitnessesBuilder {
    url: String,
    header_map: HeaderMap,
    last_n_blocks: Option<usize>,
    block: Option<u64>,
    stop: Option<CancellationToken>,
}

impl RpcBlocksAndWitnessesBuilder {
    /// Creates a new `RpcBlocksAndWitnessesBuilder` with the specified RPC URL.
    ///
    /// # Arguments
    /// * `url` - The RPC endpoint URL to connect to
    pub fn new(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    /// Adds the provided HTTP headers to the RPC client.
    ///
    /// # Arguments
    /// * `headers` - HTTP headers to include in RPC requests
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.header_map = headers;
        self
    }

    /// Sets the number of last blocks to fetch.
    ///
    /// # Arguments
    /// * `n` - Number of recent blocks to fetch (starting from the latest block)
    pub const fn last_n_blocks(mut self, n: usize) -> Self {
        self.last_n_blocks = Some(n);
        self
    }

    /// Listens to RPC blocks
    ///
    /// # Arguments
    /// * `stop` - A cancellation token to stop the block listener
    pub fn listen(mut self, stop: CancellationToken) -> Self {
        self.stop = Some(stop);
        self
    }

    /// Sets a specific block number to fetch.
    ///
    /// # Arguments
    /// * `block` - The block number to fetch
    pub const fn block(mut self, block: u64) -> Self {
        self.block = Some(block);
        self
    }

    /// Builds the configured `RpcBlocksAndWitnesses`.
    pub async fn build(self) -> Result<RpcFixtureGenerator> {
        let client = HttpClientBuilder::default()
            .set_headers(self.header_map)
            .max_response_size(1 << 30)
            .build(&self.url)
            .map_err(|e| WGError::RpcError(e.to_string()))?;

        let chain_id = EthApiClient::<(), (), (), (), (), ()>::chain_id(&client)
            .await
            .map_err(|e| WGError::RpcError(e.to_string()))?
            .ok_or(WGError::ChainIdFetchError)?;

        let chain = Chain::from_id(chain_id.to());

        let chain_config = match chain.named() {
            Some(NamedChain::Mainnet) => MAINNET.genesis.config.clone(),
            Some(NamedChain::Sepolia) => SEPOLIA.genesis.config.clone(),
            Some(NamedChain::Hoodi) => HOODI.genesis.config.clone(),
            Some(NamedChain::Holesky) => HOLESKY.genesis.config.clone(),
            _ => {
                return Err(WGError::UnsupportedChain(chain_id.to()));
            }
        };

        Ok(RpcFixtureGenerator {
            client,
            chain_config,
            last_n_blocks: self.last_n_blocks,
            block: self.block,
            stop: self.stop,
        })
    }
}

/// RPC-based witness generator that fetches blocks and witnesses from an Ethereum node.
#[derive(Debug, Clone)]
pub struct RpcFixtureGenerator {
    client: HttpClient,
    chain_config: ChainConfig,
    last_n_blocks: Option<usize>,
    block: Option<u64>,
    stop: Option<CancellationToken>,
}

#[async_trait]
impl FixtureGenerator for RpcFixtureGenerator {
    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let count = if self.last_n_blocks.is_some() || self.block.is_some() {
            let bws = self.generate().await?;
            self.save_to_path(&bws, path)?;
            bws.len()
        } else {
            self.fetch_live(path).await?
        };

        Ok(count)
    }

    /// Generates blocks and witnesses based on the configuration.
    ///
    /// Returns either the last N blocks or a specific block with their execution witnesses.
    async fn generate(&self) -> Result<Vec<Box<dyn Fixture>>> {
        // If live polling is enabled, we return an error here
        if self.stop.is_some() {
            return Err(WGError::LivePollingNotSupported);
        }

        // Handle last_n_blocks case
        if let Some(last_n_blocks) = self.last_n_blocks {
            return self.fetch_last_n_blocks(last_n_blocks).await;
        }

        // Handle one block case
        if let Some(block) = self.block {
            return Ok(vec![self.fetch_specific_block(block).await?]);
        }

        Ok(vec![])
    }
}

impl RpcFixtureGenerator {
    /// Fetches the last N blocks and their execution witnesses.
    ///
    /// # Arguments
    /// * `last_n_blocks` - Number of recent blocks to fetch
    ///
    /// # Errors
    /// Returns an error if any RPC call fails or if blocks cannot be found.
    async fn fetch_last_n_blocks(&self, last_n_blocks: usize) -> Result<Vec<Box<dyn Fixture>>> {
        if last_n_blocks == 0 {
            return Ok(vec![]);
        }

        let latest_block = EthApiClient::<
            TransactionRequest,
            Transaction,
            Block,
            Receipt,
            Header,
            TransactionSigned,
        >::block_by_number(&self.client, BlockNumberOrTag::Latest, false)
        .await
        .map_err(|e| WGError::RpcError(e.to_string()))?
        .ok_or(WGError::LatestBlockFetchError)?;

        let (block_num_start, block_num_end) = (
            std::cmp::max(0, latest_block.header.number - (last_n_blocks as u64 - 1)),
            latest_block.header.number,
        );

        let mut hashes = Vec::with_capacity(last_n_blocks);
        hashes.push((latest_block.header.number, latest_block.header.hash));
        for n in (block_num_start..block_num_end).rev() {
            let block_hash = hashes.last().unwrap().1;
            let block = EthApiClient::<
                TransactionRequest,
                Transaction,
                Block,
                Receipt,
                Header,
                TransactionSigned,
            >::block_by_hash(&self.client, block_hash, true)
            .await
            .map_err(|e| WGError::RpcError(e.to_string()))?
            .ok_or(WGError::BlockNotFoundForNumber(n))?;
            hashes.push((n, block.header.parent_hash));
        }

        let mut blocks_and_witnesses = Vec::with_capacity(hashes.len());
        for (block_num, block_hash) in hashes {
            let block = EthApiClient::<
                TransactionRequest,
                Transaction,
                Block<TransactionSigned>,
                Receipt,
                Header,
                TransactionSigned,
            >::block_by_hash(&self.client, block_hash, true)
            .await
            .map_err(|e| WGError::RpcError(e.to_string()))?
            .ok_or(WGError::BlockNotFoundForHash(block_hash.to_string()))?;

            let witness = DebugApiClient::<()>::debug_execution_witness_by_block_hash(
                &self.client,
                block_hash,
            )
            .await
            .map_err(|e| WGError::RpcError(e.to_string()))?;

            let bw = Box::new(StatelessValidationFixture {
                name: format!("rpc_block_{block_num}"),
                stateless_input: StatelessInput {
                    block: block.into_consensus(),
                    witness,
                    chain_config: self.chain_config.clone(),
                },
                success: true,
            }) as Box<dyn Fixture>;

            blocks_and_witnesses.push(bw);
        }

        Ok(blocks_and_witnesses)
    }

    /// Fetches a specific block and its execution witness.
    ///
    /// # Arguments
    /// * `block_num` - The block number to fetch
    ///
    /// # Errors
    /// Returns an error if the RPC call fails or if the block cannot be found.
    async fn fetch_specific_block(&self, block_num: u64) -> Result<Box<dyn Fixture>> {
        // Fetch the execution witness for the given block
        let witness = DebugApiClient::<()>::debug_execution_witness(
            &self.client,
            BlockNumberOrTag::Number(block_num),
        )
        .await
        .map_err(|e| WGError::RpcError(e.to_string()))?;

        // Fetch the block details
        let block =
            EthApiClient::<
                TransactionRequest,
                Transaction,
                Block<TransactionSigned>,
                Receipt,
                Header,
                TransactionSigned,
            >::block_by_number(&self.client, BlockNumberOrTag::Number(block_num), true)
            .await
            .map_err(|e| WGError::RpcError(e.to_string()))?
            .ok_or(WGError::BlockNotFoundForNumber(block_num))?;

        let bw = StatelessValidationFixture {
            name: format!("rpc_block_{block_num}"),
            stateless_input: StatelessInput {
                block: block.into_consensus(),
                witness,
                chain_config: self.chain_config.clone(),
            },
            success: true,
        };

        Ok(Box::new(bw))
    }

    /// Fetches blocks from a specific block number to the latest block and their execution witnesses.
    ///
    /// # Arguments
    /// * `block_num` - The starting block number to fetch
    ///
    /// # Returns
    /// A vector of `BlockAndWitness` objects for all blocks in the range
    ///
    /// # Errors
    ///
    /// Returns an error if any RPC call fails or if blocks cannot be found.
    async fn fetch_from_block(&self, block_num: u64) -> Result<Vec<Box<dyn Fixture>>> {
        let latest_block = EthApiClient::<
            TransactionRequest,
            Transaction,
            Block,
            Receipt,
            Header,
            TransactionSigned,
        >::block_by_number(&self.client, BlockNumberOrTag::Latest, false)
        .await
        .map_err(|e| WGError::RpcError(e.to_string()))?
        .ok_or(WGError::LatestBlockFetchError)?;

        let mut bws = Vec::new();
        for n in block_num..=latest_block.header.number {
            bws.push(self.fetch_specific_block(n).await?);
        }

        Ok(bws)
    }

    /// Continuously fetches new blocks and their execution witnesses, writing them to the specified path.
    ///
    /// This method polls for new blocks starting from the latest block and continues until
    /// the cancellation token is triggered. Each new block is processed and saved as a
    /// JSON fixture file.
    ///
    /// # Arguments
    /// * `path` - The directory path where JSON fixture files will be written
    ///
    /// # Returns
    /// The total number of blocks processed and saved
    ///
    /// # Errors
    ///
    /// Returns an error if the cancellation token is not set, if RPC calls fail,
    /// or if file writing fails.
    async fn fetch_live(&self, path: &Path) -> Result<usize> {
        let latest_block = EthApiClient::<
            TransactionRequest,
            Transaction,
            Block,
            Receipt,
            Header,
            TransactionSigned,
        >::block_by_number(&self.client, BlockNumberOrTag::Latest, false)
        .await
        .map_err(|e| WGError::RpcError(e.to_string()))?
        .ok_or(WGError::LatestBlockFetchError)?;

        let mut count: usize = 0;
        let mut next_block_num = latest_block.header.number;

        // The main loop for polling.
        let stop_signal = self
            .stop
            .as_ref()
            .ok_or(WGError::CancellationTokenRequired)?;
        loop {
            tokio::select! {
                _ = stop_signal.cancelled() => {
                    info!("Stopped listening for new blocks.");
                    break;
                }
                res = self.fetch_from_block(next_block_num) => {
                    match res {
                        Ok(bws) => {
                            if !bws.is_empty() {
                                match self.save_to_path(&bws, path) {
                                    Ok(_) => {
                                        count += bws.len();
                                        next_block_num = bws.last().unwrap().block_number() + 1;
                                    },
                                    Err(e) => error!("Failed to save data: {e}"),
                                }
                            }
                        }
                        Err(e) => {
                            error!("RPC call from block {next_block_num} failed: {e}");
                        }
                    }
                }
            }

            tokio::select! {
                _ = stop_signal.cancelled() => {
                    info!("Stopped listening for new blocks.");
                    break;
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(6)) => {
                }
            }
        }

        Ok(count)
    }

    /// Saves a collection of `BlockAndWitness` objects to JSON files in the specified directory.
    ///
    /// Each fixture is written to a separate JSON file named after the fixture's name.
    ///
    /// # Arguments
    /// * `bws` - The slice of `BlockAndWitness` objects to save
    /// * `path` - The directory path where JSON files will be written
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails or if any file cannot be written.
    fn save_to_path(&self, bws: &[Box<dyn Fixture>], path: &Path) -> Result<()> {
        for bw in bws {
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
            info!("Saved block and witness to: {}", output_path.display());
        }
        Ok(())
    }
}

/// Represents HTTP headers in a flat string format for easier configuration.
///
/// Each header should be in the format "key:value" or "key: value".
#[derive(Debug, Clone, Default)]
pub struct RpcFlatHeaderKeyValues {
    headers: Vec<String>,
}

impl RpcFlatHeaderKeyValues {
    /// Creates a new `RpcFlatHeaderKeyValues` from a vector of header strings.
    ///
    /// # Arguments
    /// * `headers` - Vector of header strings in "key:value" format
    pub const fn new(headers: Vec<String>) -> Self {
        Self { headers }
    }
}

impl TryFrom<RpcFlatHeaderKeyValues> for HeaderMap {
    type Error = WGError;

    fn try_from(flat_headers: RpcFlatHeaderKeyValues) -> Result<Self> {
        let header_pairs = flat_headers
            .headers
            .into_iter()
            .map(|header| {
                let (key, value) =
                    header
                        .split_once(':')
                        .ok_or_else(|| WGError::InvalidHeaderFormat {
                            header: header.clone(),
                        })?;

                let name =
                    HeaderName::from_str(key.trim()).map_err(|e| WGError::InvalidHeaderName {
                        name: key.to_string(),
                        source: e,
                    })?;

                let value = HeaderValue::from_str(value.trim()).map_err(|e| {
                    WGError::InvalidHeaderValue {
                        value: value.to_string(),
                        source: e,
                    }
                })?;

                Ok((name, value))
            })
            .collect::<Result<Vec<_>>>()?;

        let mut header_map = Self::with_capacity(header_pairs.len());
        for (name, value) in header_pairs {
            header_map.insert(name, value);
        }

        Ok(header_map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_base_rpc() -> RpcBlocksAndWitnessesBuilder {
        let rpc_url = std::env::var("RPC_URL").expect("RPC_URL not set");
        let rpc_headers = std::env::var("RPC_HEADERS").ok();

        let mut builder = RpcBlocksAndWitnessesBuilder::new(rpc_url);
        if let Some(rpc_headers) = rpc_headers {
            let rpc_headers: RpcFlatHeaderKeyValues = RpcFlatHeaderKeyValues::new(
                rpc_headers
                    .split(',')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            );
            builder =
                builder.with_headers(rpc_headers.try_into().expect("Failed to parse headers"));
        }
        builder
    }

    #[tokio::test]
    async fn test_last_n_blocks() {
        if std::env::var("RPC_URL").is_err() {
            eprintln!("skipping test: set RPC_URL to run this test");
            return;
        }

        let rpc_bw = build_base_rpc()
            .last_n_blocks(2)
            .build()
            .await
            .expect("Failed to build RPC Blocks and Witnesses");

        // Generate to Vector
        let bws = rpc_bw
            .generate()
            .await
            .expect("Failed to generate blocks and witnesses");

        assert_eq!(bws.len(), 2, "Expected 2 blocks and witnesses");

        // Generate to path
        let target_dir = tempfile::tempdir()
            .expect("Failed to create temporary directory for blocks and witnesses");
        rpc_bw
            .generate_to_path(target_dir.path())
            .await
            .expect("Failed to generate blocks and witnesses to path");

        assert_eq!(
            std::fs::read_dir(target_dir.path())
                .expect("Failed to read directory")
                .count(),
            2,
            "Expected 2 files in temporary directory"
        );
    }

    #[tokio::test]
    async fn test_concrete_block_num() {
        if std::env::var("RPC_URL").is_err() {
            eprintln!("skipping test: set RPC_URL to run this test");
            return;
        }

        let latest_block_number = EthApiClient::<
            TransactionRequest,
            Transaction,
            Block,
            Receipt,
            Header,
            TransactionSigned,
        >::block_by_number(
            &build_base_rpc().build().await.unwrap().client,
            BlockNumberOrTag::Latest,
            false,
        )
        .await
        .expect("Failed to fetch latest block")
        .unwrap()
        .header
        .number;

        // Fetch a non-tip block number
        let block_number = latest_block_number - 1;

        let rpc_bw = build_base_rpc()
            .block(block_number)
            .build()
            .await
            .expect("Failed to build RPC Blocks and Witnesses");

        // Generate to Vector
        let bws = rpc_bw
            .generate()
            .await
            .expect("Failed to generate blocks and witnesses");

        assert_eq!(bws.len(), 1, "Expected 1 block and witness");
        assert_eq!(
            bws[0].block_number(),
            block_number,
            "Expected block number to match"
        );

        // Generate to path
        let target_dir = tempfile::tempdir()
            .expect("Failed to create temporary directory for blocks and witnesses");
        rpc_bw
            .generate_to_path(target_dir.path())
            .await
            .expect("Failed to generate blocks and witnesses to path");

        assert_eq!(
            std::fs::read_dir(target_dir.path())
                .expect("Failed to read directory")
                .count(),
            1,
            "Expected 1 files in temporary directory"
        );
    }

    #[tokio::test]
    async fn test_live_blocks() {
        if std::env::var("RPC_URL").is_err() {
            eprintln!("skipping test: set RPC_URL to run this test");
            return;
        }

        let stop_token = CancellationToken::new();

        // Spawn a task to cancel the token after ~12s which should be enough time to fetch at least one block.
        tokio::spawn({
            let stop_token = stop_token.clone();
            async move {
                tokio::time::sleep(std::time::Duration::from_secs(12)).await;
                stop_token.cancel();
                info!("Sent cancellation signal");
            }
        });

        let target_dir = tempfile::tempdir()
            .expect("Failed to create temporary directory for blocks and witnesses");

        build_base_rpc()
            .listen(stop_token)
            .build()
            .await
            .expect("Failed to build RPC Blocks and Witnesses")
            .generate_to_path(target_dir.path())
            .await
            .expect("Failed to generate blocks and witnesses to path");

        assert!(
            std::fs::read_dir(target_dir.path())
                .expect("Failed to read directory")
                .count()
                > 0,
            "Expected at least one block"
        );
    }
}
