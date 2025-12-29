//! Stateless validator guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::{read_benchmark_fixtures_folder, BlockMetadata},
};
use guest_libs::senders::recover_signers;
use openvm_mpt::{from_proof::from_execution_witness, statelesstrie::OpenVMStatelessSparseTrie};
use reth_guest::guest::{RethStatelessValidatorGuest, RethStatelessValidatorInput};
use std::{path::Path, sync::OnceLock};
use witness_generator::StatelessValidationFixture;

/// Prepares the inputs for the Reth stateless validator guest program.
pub fn stateless_validator_inputs(
    input_folder: &Path,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    let fixtures = read_benchmark_fixtures_folder(input_folder)?;
    stateless_validator_inputs_from_fixture(&fixtures)
}

/// Create a vector of `GuestFixture` instances from `StatelessValidationFixture`.
pub fn stateless_validator_inputs_from_fixture(
    fixture: &[StatelessValidationFixture],
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    fixture
        .iter()
        .map(|bw| {
            let input = transform_witness(get_input_full_validation(bw)?);

            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            Ok(
                GenericGuestFixture::<RethStatelessValidatorGuest<OpenVMStatelessSparseTrie>, _> {
                    name: bw.name.clone(),
                    input,
                    metadata,
                    output: OnceLock::from((
                        bw.stateless_input.block.hash_slow().0,
                        bw.stateless_input.block.parent_hash.0,
                        bw.success,
                    )),
                }
                .into_output_sha256()
                .into_boxed(),
            )
        })
        .collect()
}

/// Prepares a single input for the Reth stateless validator guest program with full validation.
pub fn get_input_full_validation(
    bw: &StatelessValidationFixture,
) -> anyhow::Result<RethStatelessValidatorInput> {
    let stateless_input = &bw.stateless_input;
    let signers = recover_signers(&stateless_input.block.body.transactions)?;

    Ok(RethStatelessValidatorInput {
        stateless_input: stateless_input.clone(),
        public_keys: signers,
    })
}

/// Transforms the witness in the input from Risc0 MPT format to OpenVM MPT format.
pub fn transform_witness(mut input: RethStatelessValidatorInput) -> RethStatelessValidatorInput {
    let pre_state_root = state_root_from_headers(
        input.stateless_input.block.number - 1,
        &input.stateless_input.witness.headers,
    );
    let tries_bytes = from_execution_witness(pre_state_root, &input.stateless_input.witness)
        .unwrap()
        .encode_to_state_bytes();
    let bytes = bincode::serialize(&tries_bytes).unwrap();
    input.stateless_input.witness.state = vec![bytes.into()];
    input
}

fn state_root_from_headers(block_num: u64, headers: &[impl AsRef<[u8]>]) -> alloy_primitives::B256 {
    headers
        .iter()
        .find_map(|h| {
            let header = alloy_rlp::decode_exact::<alloy_consensus::Header>(h).unwrap();
            (header.number == block_num).then_some(header.state_root)
        })
        .unwrap()
}
