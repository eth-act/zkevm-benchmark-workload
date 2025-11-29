//! Abstracted guest program

use alloc::{boxed::Box, format, sync::Arc, vec::Vec};
use c_kzg::Blob;
use core::error::Error;
use guest_libs::blobs::BlockBodyBytes;

use alloy_primitives::FixedBytes;
use ere_io_serde::IoSerde;
use ere_platform_trait::Platform;
use reth_chainspec::ChainSpec;
use reth_ethereum_primitives::Block as EthBlock;
use reth_evm_ethereum::EthEvmConfig;
use reth_guest_io::{io_serde, BincodeBlockBody, Input};
use reth_primitives_traits::Block;
use reth_stateless::{
    stateless_validation_with_trie, ExecutionWitness, Genesis, UncompressedPublicKey,
};
use sparsestate::SparseState;

/// Main entry point for the guest program.
pub fn ethereum_guest<P: Platform>() {
    P::cycle_scope_start("read_input");
    let mut input: Input = io_serde()
        .deserialize(&P::read_whole_input())
        .expect("Failed to read input");

    let genesis = Genesis {
        config: input.stateless_input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    P::cycle_scope_end("read_input");

    if let Some(da_proof) = input.block_body.da_proof {
        P::cycle_scope_start("kzg_init");
        let kzg_settings = c_kzg::ethereum_kzg_settings(8);
        P::cycle_scope_end("kzg_init");

        P::cycle_scope_start("da_proof_verification");
        let blobs: Vec<Blob> = guest_libs::blobs::partition_into_blobs(input.block_body.bytes.as_slice());
        let commitments: Vec<_> = da_proof.kzg_commitments.into_iter().map(Into::into).collect();
        let proofs: Vec<_> = da_proof.kzg_proofs.into_iter().map(Into::into).collect();
        let proof_verifies = kzg_settings
            .verify_blob_kzg_proof_batch(&blobs, &commitments, &proofs)
            .expect("KZG DA proof verification should not fail");
        if !proof_verifies {
            panic!("KZG DA proof verification failed");
        }
        P::cycle_scope_end("da_proof_verification");
    }

    P::cycle_scope_start("block_body_decompression");
    let block_body_raw: Vec<u8> = match input.block_body.bytes {
        BlockBodyBytes::Raw(body) => body,
        BlockBodyBytes::CompressedSnappy(compressed) => snap::raw::Decoder::new()
            .decompress_vec(&compressed)
            .expect("Failed to decompress snappy-compressed block body"),
    };
    P::cycle_scope_end("block_body_decompression");

    P::cycle_scope_start("block_body_deserialization");
    let block_body: BincodeBlockBody = io_serde()
        .deserialize(&block_body_raw)
        .expect("Failed to deserialize block body");
    input.stateless_input.block.body = block_body.0;
    P::cycle_scope_end("block_body_deserialization");

    P::cycle_scope_start("public_inputs_preparation");
    let header = input.stateless_input.block.header().clone();
    let parent_hash = input.stateless_input.block.parent_hash;
    P::cycle_scope_end("public_inputs_preparation");

    let res = validate_block::<P>(
        input.stateless_input.block,
        input.stateless_input.witness,
        chain_spec,
        input.public_keys,
        evm_config,
    );
    P::cycle_scope_start("commit_public_inputs");
    let public_input_bytes = match res {
        Ok(block_hash) => {
            let public_inputs = (block_hash.0, parent_hash.0, true);
            bincode_v2::serde::encode_to_vec(public_inputs, bincode_v2::config::legacy()).unwrap()
        }
        Err(_err) => {
            P::print(&format!("Block validation failed: {_err}\n"));
            let public_inputs = (header.hash_slow().0, parent_hash.0, false);
            bincode_v2::serde::encode_to_vec(public_inputs, bincode_v2::config::legacy()).unwrap()
        }
    };
    P::write_whole_output(&public_input_bytes);
    P::cycle_scope_end("commit_public_inputs");
}

fn validate_block<P: Platform>(
    block: EthBlock,
    witness: ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    public_keys: Vec<UncompressedPublicKey>,
    evm_config: EthEvmConfig,
) -> Result<FixedBytes<32>, Box<dyn Error>> {
    P::cycle_scope_start("validation");
    let (block_hash, _) = stateless_validation_with_trie::<SparseState, _, _>(
        block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
    )?;
    P::cycle_scope_end("validation");

    Ok(block_hash)
}
