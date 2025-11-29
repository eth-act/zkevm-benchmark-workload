//! Stateless validator guest program.

use std::{convert::TryInto, path::Path};

use alloy_eips::eip6110::MAINNET_DEPOSIT_CONTRACT_ADDRESS;
use alloy_rlp::Encodable;
use anyhow::{Context, Result};
use ere_dockerized::zkVMKind;
use ere_io_serde::IoSerde;
use ethrex_common::{
    types::{
        block_execution_witness, BlobSchedule, Block, BlockHeader, ChainConfig, ForkBlobSchedule,
    },
    H160,
};
use ethrex_rlp::decode::RLPDecode;
use ethrex_rpc::debug::execution_witness::{
    execution_witness_from_rpc_chain_config, RpcExecutionWitness,
};
use guest_libs::blobs::BlockBodyEncoding;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reth_stateless::StatelessInput;
use rkyv::rancor::Error;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use strum::{AsRefStr, EnumString};
use walkdir::WalkDir;
use witness_generator::StatelessValidationFixture;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier, OutputVerifierResult};

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    block_used_gas: u64,
}
impl GuestMetadata for BlockMetadata {}

/// Analysis results for block body compression.
#[derive(Debug, Clone)]
pub struct CompressionAnalysis {
    /// Name of the fixture being analyzed.
    pub name: String,
    /// Size of the raw (uncompressed) block body in bytes.
    pub raw_size: usize,
    /// Size of the compressed block body in bytes.
    pub compressed_size: usize,
    /// Number of blobs required to store the raw block body.
    pub raw_blobs: usize,
    /// Number of blobs required to store the compressed block body.
    pub compressed_blobs: usize,
    /// Compression ratio (compressed size / raw size).
    pub compression_ratio: f64,
    /// Number of blobs saved by compression (`raw_blobs` - `compressed_blobs`).
    pub blob_savings: i32,
}

const BYTES_PER_FIELD_ELEMENT: usize = 32;
const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
const USABLE_BYTES_PER_ELEMENT: usize = BYTES_PER_FIELD_ELEMENT - 1; // High byte = 0 to stay below modulus
const USABLE_BYTES_PER_BLOB: usize = FIELD_ELEMENTS_PER_BLOB * USABLE_BYTES_PER_ELEMENT;

const fn calculate_blob_count(data_size: usize) -> usize {
    data_size.div_ceil(USABLE_BYTES_PER_BLOB)
}

/// Analyzes compression effectiveness for block bodies in benchmark fixtures.
///
/// Reads all fixtures from the input folder, serializes their block bodies,
/// compresses them using Snappy, and calculates compression metrics including
/// how many blobs would be needed before and after compression.
pub fn analyze_compression(input_folder: &Path) -> Result<Vec<CompressionAnalysis>> {
    let fixtures = read_benchmark_fixtures_folder(input_folder)?;

    fixtures
        .into_iter()
        .map(|fixture| {
            let body = reth_guest_io::BincodeBlockBody(fixture.stateless_input.block.body.clone());
            let raw_bytes = reth_guest_io::io_serde()
                .serialize(&body)
                .map_err(|e| anyhow::anyhow!("serializing block body: {e}"))?;

            let raw_size = raw_bytes.len();

            let compressed_bytes = snap::raw::Encoder::new()
                .compress_vec(&raw_bytes)
                .map_err(|e| anyhow::anyhow!("compressing block body: {e}"))?;

            let compressed_size = compressed_bytes.len();

            let raw_blobs = calculate_blob_count(raw_size);
            let compressed_blobs = calculate_blob_count(compressed_size);

            Ok(CompressionAnalysis {
                name: fixture.name,
                raw_size,
                compressed_size,
                raw_blobs,
                compressed_blobs,
                compression_ratio: compressed_size as f64 / raw_size as f64,
                blob_savings: raw_blobs as i32 - compressed_blobs as i32,
            })
        })
        .collect()
}

/// Prepares the inputs for the stateless validator guest program based on the mode.
pub fn stateless_validator_inputs(
    input_folder: &Path,
    el: ExecutionClient,
    block_body_encoding: BlockBodyEncoding,
    block_body_with_proof: bool,
) -> Result<Vec<GuestIO<BlockMetadata, ProgramOutputVerifier>>> {
    let mut res = vec![];
    let witnesses = read_benchmark_fixtures_folder(input_folder)?;
    for bw in &witnesses {
        let input = get_input_full_validation(bw, &el, block_body_encoding, block_body_with_proof)?;
        let metadata = BlockMetadata {
            block_used_gas: bw.stateless_input.block.gas_used,
        };
        let output = ProgramOutputVerifier { bw: bw.clone() };
        res.push(GuestIO {
            name: bw.name.clone(),
            input,
            metadata,
            output,
        })
    }
    Ok(res)
}

/// Reads the benchmark fixtures folder and returns a list of block and witness pairs.
pub fn read_benchmark_fixtures_folder(path: &Path) -> Result<Vec<StatelessValidationFixture>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let bw: StatelessValidationFixture =
                    serde_json::from_slice(&content).map_err(|e| {
                        anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                    })?;
                Ok(bw)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect()
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier {
    bw: StatelessValidationFixture,
}

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, _zkvm: zkVMKind, bytes: &[u8]) -> Result<OutputVerifierResult> {
        let block_hash = self.bw.stateless_input.block.hash_slow().0;
        let parent_hash = self.bw.stateless_input.block.parent_hash.0;
        let success = self.bw.success;

        let public_inputs = (block_hash, parent_hash, success);
        let public_inputs_hash = Sha256::digest(bincode::serialize(&public_inputs)?);

        if public_inputs_hash.as_slice() != bytes {
            return Ok(OutputVerifierResult::Mismatch(format!(
                "Public inputs hash mismatch: expected {public_inputs_hash:?}, got {bytes:?}"
            )));
        }

        Ok(OutputVerifierResult::Match)
    }
}

fn get_input_full_validation(
    bw: &StatelessValidationFixture,
    el: &ExecutionClient,
    block_body_encoding: BlockBodyEncoding,
    block_body_with_proof: bool,
) -> Result<Vec<u8>> {
    let si = &bw.stateless_input;
    match el {
        ExecutionClient::Reth => reth_guest_io::io_serde()
            .serialize(
                &reth_guest_io::Input::new(si.clone(), block_body_encoding, block_body_with_proof)
                    .context("Failed to create Reth input")?,
            )
            .map_err(|e| anyhow::anyhow!("Reth serialization error: {e}")),
        ExecutionClient::Ethrex => {
            let mut rlp_bytes = vec![];
            si.block.encode(&mut rlp_bytes);
            let (ethrex_block, _) = Block::decode_unfinished(&rlp_bytes)?;

            let ethrex_program_input = ethrex_guest_program::input::ProgramInput {
                blocks: vec![ethrex_block],
                execution_witness: from_reth_witness_to_ethrex_witness(si.block.number, si)?,
                elasticity_multiplier: 2u64, // NOTE: Ethrex doesn't derive this value from chain config.
                fee_configs: Default::default(),
            };

            Ok(rkyv::to_bytes::<Error>(&ethrex_program_input)?.to_vec())
        }
    }
}

fn from_reth_witness_to_ethrex_witness(
    block_number: u64,
    si: &StatelessInput,
) -> Result<block_execution_witness::ExecutionWitness> {
    let codes = si.witness.codes.iter().map(|b| b.to_vec().into()).collect();
    let block_headers_bytes = si
        .witness
        .headers
        .iter()
        .map(|h| h.to_vec().into())
        .collect();

    let chain_config = ChainConfig {
        chain_id: si.chain_config.chain_id,
        homestead_block: si.chain_config.homestead_block,
        dao_fork_block: si.chain_config.dao_fork_block,
        dao_fork_support: si.chain_config.dao_fork_support,
        eip150_block: si.chain_config.eip150_block,
        eip155_block: si.chain_config.eip155_block,
        eip158_block: si.chain_config.eip158_block,
        byzantium_block: si.chain_config.byzantium_block,
        constantinople_block: si.chain_config.constantinople_block,
        petersburg_block: si.chain_config.petersburg_block,
        istanbul_block: si.chain_config.istanbul_block,
        muir_glacier_block: si.chain_config.muir_glacier_block,
        berlin_block: si.chain_config.berlin_block,
        london_block: si.chain_config.london_block,
        arrow_glacier_block: si.chain_config.arrow_glacier_block,
        gray_glacier_block: si.chain_config.gray_glacier_block,
        merge_netsplit_block: si.chain_config.merge_netsplit_block,
        shanghai_time: si.chain_config.shanghai_time,
        cancun_time: si.chain_config.cancun_time,
        prague_time: si.chain_config.prague_time,
        verkle_time: None,
        osaka_time: si.chain_config.osaka_time,
        terminal_total_difficulty: si
            .chain_config
            .terminal_total_difficulty
            .map(|ttd| TryInto::<u128>::try_into(ttd).unwrap()),
        terminal_total_difficulty_passed: si.chain_config.terminal_total_difficulty_passed,
        blob_schedule: BlobSchedule {
            cancun: get_blob_schedule(&si.chain_config, "cancun")
                .unwrap_or_else(|| BlobSchedule::default().cancun),
            prague: get_blob_schedule(&si.chain_config, "prague")
                .unwrap_or_else(|| BlobSchedule::default().prague),
            osaka: get_blob_schedule(&si.chain_config, "osaka")
                .unwrap_or_else(|| BlobSchedule::default().osaka),
            bpo1: get_blob_schedule(&si.chain_config, "bpo1")
                .unwrap_or_else(|| BlobSchedule::default().bpo1),
            bpo2: get_blob_schedule(&si.chain_config, "bpo2")
                .unwrap_or_else(|| BlobSchedule::default().bpo2),
            bpo3: get_blob_schedule(&si.chain_config, "bpo3"),
            bpo4: get_blob_schedule(&si.chain_config, "bpo4"),
            bpo5: get_blob_schedule(&si.chain_config, "bpo5"),
        },
        deposit_contract_address: si
            .chain_config
            .deposit_contract_address
            .map(|addr| H160::from_slice(addr.as_slice()))
            .unwrap_or_else(|| H160::from_slice(MAINNET_DEPOSIT_CONTRACT_ADDRESS.as_slice())),
        bpo1_time: si.chain_config.bpo1_time,
        bpo2_time: si.chain_config.bpo2_time,
        bpo3_time: si.chain_config.bpo3_time,
        bpo4_time: si.chain_config.bpo4_time,
        bpo5_time: si.chain_config.bpo5_time,
        enable_verkle_at_genesis: false,
    };

    let nodes = si
        .witness
        .state
        .iter()
        .map(|node_rlp| node_rlp.to_vec().into())
        .collect();

    let keys = si.witness.keys.iter().map(|k| k.to_vec().into()).collect();

    let parent_hash = si.block.parent_hash;
    let initial_state_root = si
        .witness
        .headers
        .iter()
        .find_map(|header_bytes| {
            let (header, _) = BlockHeader::decode_unfinished(header_bytes).ok()?;
            (header.hash().0 == parent_hash.0).then_some(header.state_root)
        })
        .context("Parent header not found in witness")?;

    let rpc_witness = RpcExecutionWitness {
        state: nodes,
        keys,
        codes,
        headers: block_headers_bytes,
    };
    let execution_witness = execution_witness_from_rpc_chain_config(
        rpc_witness,
        chain_config,
        block_number,
        initial_state_root,
    )?;

    Ok(execution_witness)
}

fn get_blob_schedule(
    chain_config: &alloy_genesis::ChainConfig,
    name: &str,
) -> Option<ethrex_common::types::ForkBlobSchedule> {
    chain_config
        .blob_schedule
        .get(name)
        .map(|s| ForkBlobSchedule {
            // Reth and Ethrex have some mismatched data type representations. Reth uses bigger ints.
            // Downcasting should never cause an overflow, but let's be safe and panic if this ever happens.
            base_fee_update_fraction: s.update_fraction.try_into().unwrap(),
            target: s.target_blob_count.try_into().unwrap(),
            max: s.max_blob_count.try_into().unwrap(),
        })
}
