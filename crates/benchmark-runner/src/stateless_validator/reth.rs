//! Stateless validator guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::{read_benchmark_fixtures_folder, BlockMetadata},
};
use guest_libs::senders::recover_signers;
use reth_guest::guest::{RethStatelessValidatorGuest, RethStatelessValidatorInput};
use sparsestate::SparseState;
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
            let input = get_input_full_validation(bw)?;
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            Ok(
                GenericGuestFixture::<RethStatelessValidatorGuest<SparseState>, _> {
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
