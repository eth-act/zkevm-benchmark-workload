//! Abstracted guest program

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::error::Error;

use alloy_primitives::FixedBytes;
use ere_io_serde::IoSerde;
use k256::sha2::{Digest, Sha256};
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_guest_io::{io_serde, BincodeBlockBody, BlockBodyBytes, Input};
use reth_primitives_traits::Block;
use reth_stateless::{
    stateless_validation_with_trie, ExecutionWitness, Genesis, UncompressedPublicKey,
};
use sparsestate::SparseState;

use crate::sdk::{ScopeMarker, SDK};

/// Main entry point for the guest program.
pub fn ethereum_guest<S: SDK>() {
    S::cycle_scope(ScopeMarker::Start, "read_input");
    let mut input: Input = io_serde()
        .deserialize(&S::read_input())
        .expect("Failed to read input");

    let genesis = Genesis {
        config: input.stateless_input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    S::cycle_scope(ScopeMarker::End, "read_input");

    if input.kzg_enabled {
        S::cycle_scope(ScopeMarker::Start, "kzg_init");
        let kzg_settings = c_kzg::ethereum_kzg_settings(8);
        S::cycle_scope(ScopeMarker::End, "kzg_init");

        S::cycle_scope(ScopeMarker::Start, "kzg_commitments");
        let kzg_data: &[u8] = match &input.block_body_bytes {
            BlockBodyBytes::Raw(body) => body,
            BlockBodyBytes::CompressedSnappy(compressed) => compressed,
        };
        let blobs = partition_into_blobs(kzg_data);
        let _commitments: Vec<_> = blobs
            .iter()
            .map(|blob| {
                kzg_settings
                    .blob_to_kzg_commitment(blob)
                    .expect("Failed to compute KZG commitment")
            })
            .collect();
        S::cycle_scope(ScopeMarker::End, "kzg_commitments");
    }

    S::cycle_scope(ScopeMarker::Start, "block_body_decompression");
    let block_body_raw: Vec<u8> = match &input.block_body_bytes {
        BlockBodyBytes::Raw(body) => body.clone(),
        BlockBodyBytes::CompressedSnappy(compressed) => snap::raw::Decoder::new()
            .decompress_vec(compressed)
            .expect("Failed to decompress snappy-compressed block body"),
    };
    S::cycle_scope(ScopeMarker::End, "block_body_decompression");

    S::cycle_scope(ScopeMarker::Start, "block_body_deserialization");
    let block_body: BincodeBlockBody = io_serde()
        .deserialize(&block_body_raw)
        .expect("Failed to deserialize block body");
    input.stateless_input.block.body = block_body.0;
    S::cycle_scope(ScopeMarker::End, "block_body_deserialization");

    S::cycle_scope(ScopeMarker::Start, "public_inputs_preparation");
    let header = input.stateless_input.block.header().clone();
    let parent_hash = input.stateless_input.block.parent_hash;
    S::cycle_scope(ScopeMarker::End, "public_inputs_preparation");

    let res = validate_block::<S>(
        input.stateless_input.block,
        input.stateless_input.witness,
        chain_spec,
        input.public_keys,
        evm_config,
    );
    S::cycle_scope(ScopeMarker::Start, "commit_public_inputs");
    match res {
        Ok(block_hash) => {
            let public_inputs = (block_hash.0, parent_hash.0, true);
            let public_inputs_hash: [u8; 32] = Sha256::digest(
                bincode_v2::serde::encode_to_vec(public_inputs, bincode_v2::config::legacy())
                    .unwrap(),
            )
            .into();
            S::commit_output(public_inputs_hash);
        }
        Err(_err) => {
            #[cfg(feature = "std")]
            println!("Block validation failed: {_err}");
            let public_inputs = (header.hash_slow().0, parent_hash.0, false);
            let public_inputs_hash: [u8; 32] = Sha256::digest(
                bincode_v2::serde::encode_to_vec(public_inputs, bincode_v2::config::legacy())
                    .unwrap(),
            )
            .into();
            S::commit_output(public_inputs_hash);
        }
    }
    S::cycle_scope(ScopeMarker::End, "commit_public_inputs");
}

fn validate_block<S: SDK>(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<UncompressedPublicKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    S::cycle_scope(ScopeMarker::Start, "validation");
    let (block_hash, _) = stateless_validation_with_trie::<SparseState, _, _>(
        block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
    )?;
    S::cycle_scope(ScopeMarker::End, "validation");

    Ok(block_hash)
}

fn partition_into_blobs(data: &[u8]) -> Vec<c_kzg::Blob> {
    const BYTES_PER_BLOB: usize = c_kzg::BYTES_PER_BLOB;
    const USABLE_BYTES_PER_ELEMENT: usize = c_kzg::BYTES_PER_FIELD_ELEMENT - 1; // Leave high byte as 0 to stay below modulus
    const USABLE_BYTES_PER_BLOB: usize = c_kzg::FIELD_ELEMENTS_PER_BLOB * USABLE_BYTES_PER_ELEMENT;

    if data.is_empty() {
        return Vec::new();
    }

    let num_blobs = data.len().div_ceil(USABLE_BYTES_PER_BLOB);
    let mut blobs = Vec::with_capacity(num_blobs);
    let mut offset = 0;

    for _ in 0..num_blobs {
        let mut blob_data = [0u8; BYTES_PER_BLOB];

        for fe_idx in 0..c_kzg::FIELD_ELEMENTS_PER_BLOB {
            if offset >= data.len() {
                break;
            }
            let chunk_size = (data.len() - offset).min(USABLE_BYTES_PER_ELEMENT);
            let blob_offset = fe_idx * c_kzg::BYTES_PER_FIELD_ELEMENT + 1; // +1 leaves high byte as 0
            blob_data[blob_offset..blob_offset + chunk_size]
                .copy_from_slice(&data[offset..offset + chunk_size]);
            offset += chunk_size;
        }

        blobs.push(c_kzg::Blob::new(blob_data));
    }

    blobs
}
