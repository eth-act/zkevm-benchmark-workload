use crate::{BlocksAndWitnesses, witness_generator::WitnessGenerator};
use alloy_eips::BlockNumberOrTag;
use alloy_rpc_types_eth::{Block, Header, Receipt, Transaction};
use anyhow::Result;
use async_trait::async_trait;
use http::HeaderName;
use jsonrpsee::{
    http_client::{HeaderMap, HttpClient, HttpClientBuilder}, // Added HeaderMap here
    ws_client::HeaderValue,
};
use reth_ethereum_primitives::TransactionSigned;
use reth_rpc_api::{DebugApiClient, EthApiClient};
use reth_stateless::{ClientInput, fork_spec::ForkSpec};
use std::{cmp::max, str::FromStr};

/// Builder for configuring an RPC client that fetches blocks and witnesses.
#[derive(Debug, Clone, Default)]
pub struct RPCBlocksAndWitnessesBuilder {
    url: String,
    header_map: HeaderMap,
    last_n_blocks: Option<usize>,
}

impl RPCBlocksAndWitnessesBuilder {
    /// Creates a new `RPCBlocksAndWitnessesBuilder` with the specified RPC URL.
    pub fn new(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }

    /// Adds the provided HTTP headers to the RPC client.
    pub fn with_headers<S: AsRef<str>>(mut self, headers: Vec<(S, S)>) -> Result<Self> {
        self.header_map = headers
            .into_iter()
            .map(|(name, value)| {
                Ok((
                    HeaderName::from_str(name.as_ref())?,
                    HeaderValue::from_str(value.as_ref())?,
                ))
            })
            .collect::<Result<HeaderMap>>()?;
        Ok(self)
    }

    /// Sets the numbe of last blocks to fetch.
    pub fn last_n_blocks(mut self, n: usize) -> Self {
        self.last_n_blocks = Some(n);
        self
    }

    /// Builds the configured `RPCBlocksAndWitnesses`.
    pub fn build(self) -> Result<RPCBlocksAndWitnesses> {
        let client = HttpClientBuilder::default()
            .set_headers(self.header_map)
            .max_response_size(1 << 30)
            .build(&self.url)?;
        Ok(RPCBlocksAndWitnesses {
            client,
            last_n_blocks: self.last_n_blocks.unwrap_or(1),
        })
    }
}

/// RPCBlocksAndWitnesses is a witness generator that fetches blocks and witnesses
#[derive(Debug, Clone)]
pub struct RPCBlocksAndWitnesses {
    client: HttpClient,
    last_n_blocks: usize,
}

#[async_trait]
impl WitnessGenerator for RPCBlocksAndWitnesses {
    async fn generate(&self) -> Result<Vec<BlocksAndWitnesses>> {
        if self.last_n_blocks == 0 {
            return Ok(vec![]);
        }

        let latest_block = EthApiClient::<Transaction, Block, Receipt, Header>::block_by_number(
            &self.client,
            BlockNumberOrTag::Latest,
            false,
        )
        .await?
        .ok_or(anyhow::anyhow!("No block found"))?;

        let (block_num_start, block_num_end) = (
            max(
                0,
                latest_block.header.number - (self.last_n_blocks as u64 - 1),
            ),
            latest_block.header.number,
        );

        let mut hashes = Vec::with_capacity(self.last_n_blocks);
        hashes.push((latest_block.header.number, latest_block.header.hash));
        for n in (block_num_start..block_num_end).rev() {
            let block_hash = hashes.last().unwrap().1;
            let block = EthApiClient::<Transaction, Block, Receipt, Header>::block_by_hash(
                &self.client,
                block_hash,
                true,
            )
            .await?
            .ok_or(anyhow::anyhow!("No block found for number {}", n))?;
            hashes.push((n, block.header.parent_hash));
        }

        let mut blocks_and_witnesses = Vec::with_capacity(hashes.len());
        for (block_num, block_hash) in hashes {
            let witness = self
                .client
                // TODO: our current reth RPC version doesn't support calling `debug_executionWitnessByHash`.
                // Temporarily use the "by number" version which is potentially incorrect if a reorg happens.
                .debug_execution_witness(BlockNumberOrTag::Number(block_num))
                .await?;
            let block =
                EthApiClient::<Transaction, Block<TransactionSigned>, Receipt, Header>::block_by_hash(
                    &self.client,
                    block_hash,
                    true,
                )
                .await?
                .ok_or(anyhow::anyhow!("No block found for hash {}", block_hash))?;

            blocks_and_witnesses.push(BlocksAndWitnesses {
                name: format!("mainnet_block_{}", block_num),
                blocks_and_witnesses: vec![ClientInput {
                    block: block.into_consensus(),
                    witness,
                }],
                // FIXME: this should be dynamic based on the block, but might be useful to see if the stateless
                // reth crate can help with this probably avoiding the ForkSpec enum and using the existing
                // HardForks enum.
                network: ForkSpec::Prague,
            })
        }

        Ok(blocks_and_witnesses)
    }
}
