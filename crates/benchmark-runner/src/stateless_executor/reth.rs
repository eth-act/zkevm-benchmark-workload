//! Stateless executor guest program for Reth (execution-only, no validation).

use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_executor::{read_benchmark_fixtures_folder, BlockMetadata},
};
use guest_libs::senders::recover_signers;
use reth_stateless_executor::guest::{RethStatelessExecutorGuest, RethStatelessExecutorInput};
use std::{path::Path, sync::OnceLock};
use witness_generator::StatelessExecutorFixture;

/// Prepares the inputs for the Reth stateless executor guest program.
pub fn stateless_executor_inputs(
    input_folder: &Path,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    stateless_executor_inputs_from_fixtures(read_benchmark_fixtures_folder(input_folder)?)
}

/// Prepares the inputs from pre-loaded fixtures.
pub fn stateless_executor_inputs_from_fixtures(
    fixtures: Vec<StatelessExecutorFixture>,
) -> anyhow::Result<Vec<Box<dyn GuestFixture>>> {
    fixtures
        .into_iter()
        .map(|bw| {
            let input = get_input_execution(&bw)?;
            let metadata = BlockMetadata {
                block_used_gas: bw.stateless_input.block.gas_used,
            };

            // For execution-only, the expected output is just a boolean
            Ok(GenericGuestFixture::<RethStatelessExecutorGuest, _> {
                name: bw.name.clone(),
                input,
                metadata,
                output: OnceLock::from(bw.success),
            }
            .into_output_sha256()
            .into_boxed())
        })
        .collect()
}

fn get_input_execution(
    bw: &StatelessExecutorFixture,
) -> anyhow::Result<RethStatelessExecutorInput> {
    let stateless_input = &bw.stateless_input;
    let signers = recover_signers(&stateless_input.block.body.transactions)?;

    Ok(RethStatelessExecutorInput {
        stateless_input: stateless_input.clone(),
        public_keys: signers,
    })
}

