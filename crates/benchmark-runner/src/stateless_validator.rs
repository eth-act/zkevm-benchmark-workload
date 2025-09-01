//! Stateless validator guest program.

use std::{collections::HashMap, path::Path};

use alloy_primitives::keccak256;
use alloy_rlp::Encodable;
use anyhow::Result;
use bytes::Bytes;
use ere_dockerized::ErezkVM;
use ethrex_common::{
    types::{block_execution_witness::ExecutionWitnessResult, Block, BlockHeader},
    H256,
};
use ethrex_rlp::decode::RLPDecode;
use ethrex_trie::NodeRLP;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reth_stateless::ExecutionWitness;
use reth_stateless::StatelessInput;
use rkyv::rancor::Error;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use witness_generator::BlockAndWitness;
use zkvm_interface::Input;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier};

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
            let mut stdin = Input::new();

            let mut rlp_bytes = vec![];
            si.block.encode(&mut rlp_bytes);
            let (ethrex_block, _) = Block::decode_unfinished(&rlp_bytes)?;

            let ethrex_program_input = ethrex_zkvm_interface::io::ProgramInput {
                blocks: vec![ethrex_block],
                db: from_reth_witness_to_ethrex_witness(si.block.number, &si.witness)?,
                elasticity_multiplier: 2u64, // NOTE: Ethrex doesn't derive this value from chain config.
            };
            stdin.write_bytes(rkyv::to_bytes::<Error>(&ethrex_program_input)?.to_vec());

            Ok(stdin)
        }
    }
}

fn from_reth_witness_to_ethrex_witness(
    block_number: u64,
    value: &ExecutionWitness,
) -> Result<ExecutionWitnessResult> {
    let codes: HashMap<H256, Bytes> = value
        .codes
        .iter()
        .map(|b| (H256::from(keccak256(b).0), Bytes::from(b.clone())))
        .collect();

    let block_headers = value
        .headers
        .iter()
        .map(|h| Ok(BlockHeader::decode(h.as_ref())?))
        .map(|h| h.map(|h| (h.number, h)))
        .collect::<Result<HashMap<u64, BlockHeader>>>()?;

    let parent_block_header = block_headers.get(&(block_number - 1)).unwrap().clone();

    let chain_config = ethrex_config::networks::Network::PublicNetwork(
        ethrex_config::networks::PublicNetwork::Mainnet,
    )
    .get_genesis()?
    .config;

    let state_nodes: HashMap<H256, NodeRLP> = value
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
