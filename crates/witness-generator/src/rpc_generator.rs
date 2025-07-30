use crate::{BlockAndWitness, blocks_and_witnesses::WitnessGenerator};
use alloy_eips::BlockNumberOrTag;
use alloy_rpc_types_eth::{Block, Header, Receipt, Transaction, TransactionRequest};
use anyhow::{Context, Result};
use async_trait::async_trait;
use guest_libs::chainconfig::ChainConfig;
use http::{HeaderName, HeaderValue};
use jsonrpsee::{
    http_client::{HeaderMap, HttpClient, HttpClientBuilder},
    tracing::{error, info},
};
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
    pub fn build(self) -> Result<RpcBlocksAndWitnesses> {
        let client = HttpClientBuilder::default()
            .set_headers(self.header_map)
            .max_response_size(1 << 30)
            .build(&self.url)?;

        Ok(RpcBlocksAndWitnesses {
            client,
            // TODO: make this dynamic based on the RPC
            chain_config: ChainConfig::Mainnet,
            last_n_blocks: self.last_n_blocks,
            block: self.block,
            stop: self.stop,
        })
    }
}

/// RPC-based witness generator that fetches blocks and witnesses from an Ethereum node.
#[derive(Debug, Clone)]
pub struct RpcBlocksAndWitnesses {
    client: HttpClient,
    chain_config: ChainConfig,
    last_n_blocks: Option<usize>,
    block: Option<u64>,
    stop: Option<CancellationToken>,
}

#[async_trait]
impl WitnessGenerator for RpcBlocksAndWitnesses {
    /// Generates blocks and witnesses based on the configuration.
    ///
    /// Returns either the last N blocks or a specific block with their execution witnesses.
    async fn generate(&self) -> Result<Vec<BlockAndWitness>> {
        // If live polling is enabled, we return an error here
        if self.stop.is_some() {
            return Err(anyhow::anyhow!(
                "Live polling is not supported in generate method. Use generate_to_path instead."
            ));
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

    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let count = if self.last_n_blocks.is_some() || self.block.is_some() {
            let bws = self.generate().await?;
            self.save_to_path(&bws, path)?;
            1
        } else {
            self.fetch_live(path)
                .await
                .with_context(|| "Failed to fetch live blocks and witnesses")?
        };

        Ok(count)
    }
}

impl RpcBlocksAndWitnesses {
    /// Fetches the last N blocks and their execution witnesses.
    ///
    /// # Arguments
    /// * `last_n_blocks` - Number of recent blocks to fetch
    ///
    /// # Errors
    /// Returns an error if any RPC call fails or if blocks cannot be found.
    async fn fetch_last_n_blocks(&self, last_n_blocks: usize) -> Result<Vec<BlockAndWitness>> {
        if last_n_blocks == 0 {
            return Ok(vec![]);
        }

        let latest_block = EthApiClient::<TransactionRequest, Transaction, Block, Receipt, Header>::block_by_number(
            &self.client,
            BlockNumberOrTag::Latest,
            false,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch latest block"))?;

        let (block_num_start, block_num_end) = (
            std::cmp::max(0, latest_block.header.number - (last_n_blocks as u64 - 1)),
            latest_block.header.number,
        );

        let mut hashes = Vec::with_capacity(last_n_blocks);
        hashes.push((latest_block.header.number, latest_block.header.hash));
        for n in (block_num_start..block_num_end).rev() {
            let block_hash = hashes.last().unwrap().1;
            let block = EthApiClient::<TransactionRequest, Transaction, Block, Receipt, Header>::block_by_hash(
                &self.client,
                block_hash,
                true,
            )
            .await?
            .ok_or_else(|| anyhow::anyhow!("No block found for number {}", n))?;
            hashes.push((n, block.header.parent_hash));
        }

        let mut blocks_and_witnesses = Vec::with_capacity(hashes.len());
        for (block_num, block_hash) in hashes {
            let witness = self
                .client
                .debug_execution_witness_by_block_hash(block_hash)
                .await?;
            let block = EthApiClient::<
                TransactionRequest,
                Transaction,
                Block<TransactionSigned>,
                Receipt,
                Header,
            >::block_by_hash(&self.client, block_hash, true)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No block found for hash {}", block_hash))?;

            blocks_and_witnesses.push(BlockAndWitness {
                name: format!("rpc_block_{block_num}"),
                block_and_witness: StatelessInput {
                    block: block.into_consensus(),
                    witness,
                },
                chain_config: self.chain_config,
            });
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
    async fn fetch_specific_block(&self, block_num: u64) -> Result<BlockAndWitness> {
        // Fetch the execution witness for the given block
        let witness = self
            .client
            .debug_execution_witness(BlockNumberOrTag::Number(block_num))
            .await?;

        // Fetch the block details
        let block =
            EthApiClient::<
                TransactionRequest,
                Transaction,
                Block<TransactionSigned>,
                Receipt,
                Header,
            >::block_by_number(&self.client, BlockNumberOrTag::Number(block_num), true)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No block found for number {}", block_num))?;

        let bw = BlockAndWitness {
            name: format!("rpc_block_{block_num}"),
            block_and_witness: StatelessInput {
                block: block.into_consensus(),
                witness,
            },
            chain_config: self.chain_config,
        };

        Ok(bw)
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
    async fn fetch_from_block(&self, block_num: u64) -> Result<Vec<BlockAndWitness>> {
        let latest_block = EthApiClient::<TransactionRequest, Transaction, Block, Receipt, Header>::block_by_number(
            &self.client,
            BlockNumberOrTag::Latest,
            false,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch latest block"))?;

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
        let latest_block = EthApiClient::<TransactionRequest, Transaction, Block, Receipt, Header>::block_by_number(
            &self.client,
            BlockNumberOrTag::Latest,
            false,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to fetch latest block"))?;

        let mut count: usize = 0;
        let mut next_block_num = latest_block.header.number;

        // The main loop for polling.
        let stop_signal = self
            .stop
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Cancellation token is required for live polling"))?;
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
                                        next_block_num = bws.last().unwrap().block_and_witness.block.number + 1;
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
    fn save_to_path(&self, bws: &[BlockAndWitness], path: &Path) -> Result<()> {
        for bw in bws {
            let output_path = path.join(format!("{}.json", bw.name));
            let output_data = serde_json::to_string_pretty(&bw)
                .with_context(|| format!("Failed to serialize fixture: {}", bw.name))?;

            std::fs::write(&output_path, output_data)
                .with_context(|| format!("Failed to write fixture to: {output_path:?}"))?;
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
    type Error = anyhow::Error;

    fn try_from(flat_headers: RpcFlatHeaderKeyValues) -> Result<Self, Self::Error> {
        let header_pairs = flat_headers
            .headers
            .into_iter()
            .map(|header| {
                let (key, value) = header.split_once(':').ok_or_else(|| {
                    anyhow::anyhow!("Invalid header format: '{header}'. Expected 'key:value'")
                })?;

                let name = HeaderName::from_str(key.trim())
                    .map_err(|e| anyhow::anyhow!("Invalid header name '{key}': {e}"))?;

                let value = HeaderValue::from_str(value.trim())
                    .map_err(|e| anyhow::anyhow!("Invalid header value '{value}': {e}"))?;

                Ok((name, value))
            })
            .collect::<Result<Vec<_>, Self::Error>>()?;

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
        >::block_by_number(
            &build_base_rpc().build().unwrap().client,
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
            .expect("Failed to build RPC Blocks and Witnesses");

        // Generate to Vector
        let bws = rpc_bw
            .generate()
            .await
            .expect("Failed to generate blocks and witnesses");

        assert_eq!(bws.len(), 1, "Expected 1 block and witness");
        assert_eq!(
            bws[0].block_and_witness.block.number, block_number,
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
