//! Stateless validator guest program.

use std::{collections::HashMap, convert::TryInto, path::Path};

use alloy_eips::eip6110::MAINNET_DEPOSIT_CONTRACT_ADDRESS;
use alloy_primitives::keccak256;
use alloy_rlp::Encodable;
use anyhow::Result;
use bytes::Bytes;
use ere_dockerized::ErezkVM;
use ethrex_common::{
    types::{
        block_execution_witness::ExecutionWitnessResult, BlobSchedule, Block, BlockHeader,
        ChainConfig, ForkBlobSchedule,
    },
    H160, H256,
};
use ethrex_rlp::decode::RLPDecode;
use ethrex_trie::NodeRLP;
use ethrex_zkvm_interface::io::ProgramInput;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reth_stateless::StatelessInput;
use rkyv::rancor::Error;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};
use walkdir::WalkDir;
use witness_generator::BlockAndWitness;
use zkvm_interface::Input;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier};

/// Execution client variants.
#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr)]
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

/// Generate inputs for the stateless validator guest program.
pub fn stateless_validator_inputs(
    input_folder: &Path,
    el: ExecutionClient,
) -> Result<Vec<GuestIO<BlockMetadata, ProgramOutputVerifier>>> {
    let guest_inputs = read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            Ok(GuestIO {
                name: bw.name,
                input: write_stdin(&bw.block_and_witness, &el)?,
                metadata: BlockMetadata {
                    block_used_gas: bw.block_and_witness.block.gas_used,
                },
                output: ProgramOutputVerifier {
                    block_hash: bw.block_and_witness.block.hash_slow().0,
                    parent_hash: bw.block_and_witness.block.parent_hash.0,
                    success: bw.success,
                },
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(guest_inputs)
}

/// Reads the benchmark fixtures folder and returns a list of block and witness pairs.
pub fn read_benchmark_fixtures_folder(path: &Path) -> Result<Vec<BlockAndWitness>> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let bw: BlockAndWitness = serde_json::from_slice(&content).map_err(|e| {
                    anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                })?;
                Ok(bw)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect::<Result<Vec<BlockAndWitness>>>()
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier {
    block_hash: [u8; 32],
    parent_hash: [u8; 32],
    success: bool,
}

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, zkvm: ErezkVM, bytes: &[u8]) -> Result<bool> {
        let (block_hash, parent_hash, success) = match zkvm {
            ErezkVM::SP1 | ErezkVM::Risc0 => {
                let mut bytes: &[u8] = bytes;
                let block_hash: [u8; 32] = zkvm.deserialize_from(&mut bytes)?;
                let parent_hash: [u8; 32] = zkvm.deserialize_from(&mut bytes)?;
                let success: bool = zkvm.deserialize_from(&mut bytes)?;
                (block_hash, parent_hash, success)
            }
            _ => unimplemented!(),
        };
        if block_hash != self.block_hash {
            anyhow::bail!(
                "Block hash mismatch: expected {:?}, got {:?}",
                self.block_hash,
                block_hash
            );
        }
        if parent_hash != self.parent_hash {
            anyhow::bail!(
                "Parent hash mismatch: expected {:?}, got {:?}",
                self.parent_hash,
                parent_hash
            );
        }
        if success != self.success {
            anyhow::bail!(
                "Success mismatch: expected {:?}, got {:?}",
                self.success,
                success
            );
        }

        Ok(true)
    }
}

fn write_stdin(si: &StatelessInput, el: &ExecutionClient) -> Result<Input> {
    match el {
        ExecutionClient::Reth => {
            let mut stdin = Input::new();
            stdin.write(si.clone());
            Ok(stdin)
        }
        ExecutionClient::Ethrex => {
            let mut rlp_bytes = vec![];
            si.block.encode(&mut rlp_bytes);
            let (ethrex_block, _) = Block::decode_unfinished(&rlp_bytes)?;

            let ethrex_program_input = ProgramInput {
                blocks: vec![ethrex_block],
                db: from_reth_witness_to_ethrex_witness(si.block.number, si)?,
                elasticity_multiplier: 2u64, // NOTE: Ethrex doesn't derive this value from chain config.
            };

            let mut stdin = Input::new();
            stdin.write_bytes(rkyv::to_bytes::<Error>(&ethrex_program_input)?.to_vec());

            Ok(stdin)
        }
    }
}

fn from_reth_witness_to_ethrex_witness(
    block_number: u64,
    si: &StatelessInput,
) -> Result<ExecutionWitnessResult> {
    let codes: HashMap<H256, Bytes> = si
        .witness
        .codes
        .iter()
        .map(|b| (H256::from(keccak256(b).0), Bytes::from(b.clone())))
        .collect();

    let block_headers = si
        .witness
        .headers
        .iter()
        .map(|h| Ok(BlockHeader::decode(h.as_ref())?))
        .map(|h| h.map(|h| (h.number, h)))
        .collect::<Result<HashMap<u64, BlockHeader>>>()?;

    let parent_block_header = block_headers
        .get(&(block_number - 1))
        .ok_or_else(|| anyhow::anyhow!("Missing parent block header"))?
        .clone();

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
            cancun: si
                .chain_config
                .blob_schedule
                .get("Cancun")
                .map(|s| ForkBlobSchedule {
                    // Reth and Ethrex have some mismatched data type representations. Reth uses bigger ints.
                    // Downcasting should never cause an overflow, but let's be safe and panic if this ever happens.
                    base_fee_update_fraction: s.update_fraction.try_into().unwrap(),
                    target: s.target_blob_count.try_into().unwrap(),
                    max: s.max_blob_count.try_into().unwrap(),
                })
                .unwrap_or_else(|| BlobSchedule::default().cancun),
            prague: si
                .chain_config
                .blob_schedule
                .get("prague")
                .map(|s| ForkBlobSchedule {
                    // Reth and Ethrex have some mismatched data type representations. Reth uses bigger ints.
                    // Downcasting should never cause an overflow, but let's be safe and panic if this ever happens.
                    base_fee_update_fraction: s.update_fraction.try_into().unwrap(),
                    target: s.target_blob_count.try_into().unwrap(),
                    max: s.max_blob_count.try_into().unwrap(),
                })
                .unwrap_or_else(|| BlobSchedule::default().prague),
        },
        deposit_contract_address: si
            .chain_config
            .deposit_contract_address
            .map(|addr| H160::from_slice(addr.as_slice()))
            .unwrap_or_else(|| H160::from_slice(MAINNET_DEPOSIT_CONTRACT_ADDRESS.as_slice())),
    };

    let state_nodes: HashMap<H256, NodeRLP> = si
        .witness
        .state
        .iter()
        .map(|node_rlp| (H256::from(keccak256(node_rlp).0), node_rlp.clone().to_vec()))
        .collect();

    Ok(ExecutionWitnessResult {
        codes,
        block_headers,
        parent_block_header,
        chain_config,
        state_nodes,
        ..Default::default() // The rest of fields are optional
    })
}
