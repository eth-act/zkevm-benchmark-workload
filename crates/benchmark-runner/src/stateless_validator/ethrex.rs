//! Stateless validator guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture, OutputHashedGuestFixture},
    stateless_validator::{read_benchmark_fixtures_folder, BlockMetadata},
};
use alloy_eips::eip6110::MAINNET_DEPOSIT_CONTRACT_ADDRESS;
use alloy_rlp::Encodable;
use anyhow::Context;
use ethrex_common::{
    types::{
        block_execution_witness, BlobSchedule, Block, BlockHeader, ChainConfig, ForkBlobSchedule,
    },
    H160,
};
use ethrex_guest::guest::{EthrexStatelessValidatorGuest, EthrexStatelessValidatorInput};
use ethrex_guest_program::input::ProgramInput;
use ethrex_rlp::decode::RLPDecode;
use ethrex_rpc::debug::execution_witness::{
    execution_witness_from_rpc_chain_config, RpcExecutionWitness,
};
use reth_stateless::StatelessInput;
use sha2::Sha256;
use std::{convert::TryInto, path::Path, sync::OnceLock};
use witness_generator::StatelessValidationFixture;

/// Prepares the inputs for the Ethrex stateless validator guest program.
pub fn stateless_validator_inputs(
    input_folder: &Path,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    read_benchmark_fixtures_folder(input_folder)?
        .into_iter()
        .map(|bw| {
            let input = get_input_full_validation(&bw)?;
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };
            Ok(
                OutputHashedGuestFixture::<_, Sha256>::new(GenericGuestFixture::<
                    EthrexStatelessValidatorGuest,
                    _,
                > {
                    name: bw.name.clone(),
                    input,
                    metadata,
                    output: OnceLock::from((
                        bw.stateless_input.block.hash_slow().0,
                        bw.stateless_input.block.parent_hash.0,
                        bw.success,
                    )),
                })
                .into_boxed(),
            )
        })
        .collect()
}

fn get_input_full_validation(
    bw: &StatelessValidationFixture,
) -> anyhow::Result<EthrexStatelessValidatorInput> {
    let si = &bw.stateless_input;

    let mut rlp_bytes = vec![];
    si.block.encode(&mut rlp_bytes);
    let (ethrex_block, _) = Block::decode_unfinished(&rlp_bytes)?;

    let ethrex_program_input = ProgramInput {
        blocks: vec![ethrex_block],
        execution_witness: from_reth_witness_to_ethrex_witness(si.block.number, si)?,
        elasticity_multiplier: 2u64, // NOTE: Ethrex doesn't derive this value from chain config.
        fee_configs: Default::default(),
    };

    Ok(EthrexStatelessValidatorInput(ethrex_program_input))
}

fn from_reth_witness_to_ethrex_witness(
    block_number: u64,
    si: &StatelessInput,
) -> anyhow::Result<block_execution_witness::ExecutionWitness> {
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
