use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::{
        eest::EestStatelessFixture, fixtures::BenchmarkFixture, BlockMetadata, ExecutionClient,
    },
};
use anyhow::{Context, Result};
use ere_dockerized::Input;
use ere_guests_guest::Guest;
use ere_guests_integration_tests::NoopPlatform;
use ere_guests_stateless_validator_ethrex::{
    guest::StatelessValidatorEthrexGuest,
    host::{build_eip8025_input, Eip8025InputSource},
};
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use tracing::info;
use witness_generator::StatelessValidationFixture;

#[derive(Debug, Clone, Serialize)]
struct EestBlockMetadata {
    fixture_format: &'static str,
    original_test_name: String,
    source_path: String,
    block_index: usize,
    network: String,
    chain_id: u64,
    block_number: Option<u64>,
    block_used_gas: Option<u64>,
}

pub(crate) fn stateless_validator_input_from_fixture(
    fixture: BenchmarkFixture,
    el: ExecutionClient,
) -> Result<Box<dyn GuestFixture>> {
    match fixture {
        BenchmarkFixture::Legacy(fixture) => match el {
            ExecutionClient::Reth => reth_input_from_fixture(*fixture),
            ExecutionClient::Ethrex => ethrex_input_from_fixture(*fixture),
        },
        BenchmarkFixture::Eest(fixture) => raw_eest_input_from_fixture(fixture),
    }
}

fn raw_eest_input_from_fixture(fixture: EestStatelessFixture) -> Result<Box<dyn GuestFixture>> {
    let metadata = EestBlockMetadata {
        fixture_format: "eest",
        original_test_name: fixture.original_test_name,
        source_path: fixture.source_path,
        block_index: fixture.block_index,
        network: fixture.network,
        chain_id: fixture.chain_id,
        block_number: fixture.block_number,
        block_used_gas: fixture.block_used_gas,
    };
    let expected_public_values = Sha256::digest(fixture.stateless_output_bytes).to_vec();

    Ok(GenericGuestFixture::<EestBlockMetadata> {
        name: fixture.name,
        input: Input::new().with_stdin(fixture.stateless_input_bytes),
        expected_public_values,
        metadata,
    }
    .into_boxed())
}

fn ethrex_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    let input = build_eip8025_input(Eip8025InputSource::Legacy {
        stateless_input: &stateless_input,
        valid_block: success,
    })
    .context("Failed to create Ethrex stateless validator input")?;
    let output = StatelessValidatorEthrexGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorEthrexGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}

fn reth_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    info!(
        "Preparing Reth stateless validator input for fixture {}",
        name
    );
    let input = StatelessValidatorRethInput::new(&stateless_input, success)
        .context("Failed to create Reth stateless validator input")?;

    let output = StatelessValidatorRethGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}
