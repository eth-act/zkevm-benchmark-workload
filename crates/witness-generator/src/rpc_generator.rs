use crate::{BlocksAndWitnesses, blocks_and_witnesses::WitnessGenerator};
use alloy_eips::BlockNumberOrTag;
use alloy_rpc_types_eth::{Block, Header, Receipt, Transaction, TransactionRequest};
use anyhow::Result;
use async_trait::async_trait;
use http::{HeaderName, HeaderValue};
use jsonrpsee::http_client::{HeaderMap, HttpClient, HttpClientBuilder};
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_api::{DebugApiClient, EthApiClient};
use reth_stateless::{StatelessInput, fork_spec::ForkSpec};
use std::str::FromStr;

/// Builder for configuring an RPC client that fetches blocks and witnesses.
#[derive(Debug, Clone, Default)]
pub struct RpcBlocksAndWitnessesBuilder {
    url: String,
    header_map: HeaderMap,
    last_n_blocks: Option<usize>,
    block: Option<u64>,
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
            last_n_blocks: self.last_n_blocks,
            block: self.block,
        })
    }
}

/// RPC-based witness generator that fetches blocks and witnesses from an Ethereum node.
#[derive(Debug, Clone)]
pub struct RpcBlocksAndWitnesses {
    client: HttpClient,
    last_n_blocks: Option<usize>,
    block: Option<u64>,
}

#[async_trait]
impl WitnessGenerator for RpcBlocksAndWitnesses {
    /// Generates blocks and witnesses based on the configuration.
    ///
    /// Returns either the last N blocks or a specific block with their execution witnesses.
    async fn generate(&self) -> Result<Vec<BlocksAndWitnesses>> {
        // Handle last_n_blocks case
        if let Some(last_n_blocks) = self.last_n_blocks {
            return self.fetch_last_n_blocks(last_n_blocks).await;
        }

        // Handle one block case
        if let Some(block) = self.block {
            return self.fetch_specific_block(block).await;
        }

        Ok(vec![])
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
    async fn fetch_last_n_blocks(&self, last_n_blocks: usize) -> Result<Vec<BlocksAndWitnesses>> {
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

            blocks_and_witnesses.push(BlocksAndWitnesses {
                name: format!("rpc_block_{block_num}"),
                blocks_and_witnesses: vec![StatelessInput {
                    block: block.into_consensus(),
                    witness,
                }],
                // FIXME: this should be dynamic based on the block, but might be useful to see if the stateless
                // reth crate can help with this probably avoiding the ForkSpec enum and using the existing
                // HardForks enum.
                network: ForkSpec::Prague,
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
    async fn fetch_specific_block(&self, block_num: u64) -> Result<Vec<BlocksAndWitnesses>> {
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

        let blocks_and_witnesses = vec![BlocksAndWitnesses {
            name: format!("rpc_block_{}", block_num),
            blocks_and_witnesses: vec![StatelessInput {
                block: block.into_consensus(),
                witness,
            }],
            // FIXME: this should be dynamic based on the block, but might be useful to see if the stateless
            // reth crate can help with this probably avoiding the ForkSpec enum and using the existing
            // HardForks enum.
            network: ForkSpec::Prague,
        }];

        Ok(blocks_and_witnesses)
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
    pub fn new(headers: Vec<String>) -> Self {
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

        let mut header_map = HeaderMap::with_capacity(header_pairs.len());
        for (name, value) in header_pairs {
            header_map.insert(name, value);
        }

        Ok(header_map)
    }
}
