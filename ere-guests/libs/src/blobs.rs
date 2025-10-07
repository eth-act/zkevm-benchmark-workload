//! Utilities for working with blob versioned hashes in Ethereum blocks.
//!
//! This module is currently used as a temporary workaround to allow proper validation of CL kzg blob commitments
//! against the execution block, until this is properly implemented as a formal EL header field.

use alloy_genesis::ChainConfig;
use k256::sha2::{Digest, Sha256};
use reth_ethereum_primitives::Block;

/// Calculates the hash of all versioned hashes in a block if Cancun is active at the block's number and timestamp.
pub fn calculate_versioned_hashes_hash(
    chain_config: &ChainConfig,
    block: &Block,
) -> Option<[u8; 32]> {
    chain_config
        .is_cancun_active_at_block_and_timestamp(block.number, block.timestamp)
        .then_some(
            Sha256::digest(block.body.blob_versioned_hashes_iter().fold(
                Vec::new(),
                |mut acc, hash| {
                    acc.extend_from_slice(&hash.0);
                    acc
                },
            ))
            .into(),
        )
}
