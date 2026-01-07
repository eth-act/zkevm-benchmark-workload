//! Stateless validator guest program.

use crate::{
    guest_programs::{GenericGuestFixture, GenericGuestFixture2, GuestFixture},
    stateless_validator::{read_benchmark_fixtures_folder, BlockMetadata},
};
use anyhow::Context;
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorOutput, StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use guest_libs::senders::recover_signers;
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
            let input = StatelessValidatorRethInput::new(&bw.stateless_input)
                .context("Failed to create Reth stateless validator input")?;
            let output = StatelessValidatorOutput::new(
                bw.stateless_input.block.hash_slow(),
                bw.stateless_input.block.parent_hash,
                bw.success,
            );
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            let fixture =
                GenericGuestFixture2::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
                    bw.name.clone(),
                    input,
                    output,
                    metadata,
                )
                .output_sha256();

            Ok(fixture.into_boxed())
        })
        .collect()
}
