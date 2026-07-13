use crate::{
    guest_programs::{GenericGuestFixture, GuestFixture},
    stateless_validator::{
        eest::EestStatelessFixture, fixtures::BenchmarkFixture, BlockMetadata, ExecutionClient,
    },
};
use anyhow::{bail, Context, Result};
use ere_dockerized::Input;
use serde::Serialize;
use stateless_validator_common::guest::input::{ProtocolFork, StatelessInput};
use stateless_validator_zilkworm::StatelessValidatorZilkwormInput;
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
            ExecutionClient::Zilkworm => zilkworm_input_from_fixture(*fixture),
            ExecutionClient::Reth | ExecutionClient::Ethrex | ExecutionClient::Zesu => {
                bail!(
                    "{el:?} supports only canonical blockchain_tests fixtures with statelessInputBytes/statelessOutputBytes; legacy stateless_input fixtures are supported only for Zilkworm"
                )
            }
        },
        BenchmarkFixture::Eest(fixture) => match el {
            ExecutionClient::Reth | ExecutionClient::Ethrex => raw_eest_input_from_fixture(fixture),
            ExecutionClient::Zesu => zesu_input_from_fixture(fixture),
            ExecutionClient::Zilkworm => {
                bail!("EEST fixture format not yet supported for Zilkworm")
            }
        },
    }
}

fn zesu_input_from_fixture(fixture: EestStatelessFixture) -> Result<Box<dyn GuestFixture>> {
    let input = StatelessInput::from_schema_prefixed_ssz(&fixture.stateless_input_bytes)
        .with_context(|| {
            format!(
                "failed to decode canonical stateless input for Zesu fixture {}",
                fixture.name
            )
        })?;
    let fork = input.chain_config.active_fork.fork;
    if fork != ProtocolFork::Amsterdam {
        bail!(
            "Zesu {} supports only Glamsterdam inputs (ProtocolFork::Amsterdam), but fixture {} targets {fork:?}",
            ExecutionClient::Zesu.version(),
            fixture.name
        );
    }

    raw_eest_input_from_fixture(fixture)
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
    let fixture = GenericGuestFixture::<EestBlockMetadata> {
        name: fixture.name,
        input: Input::new().with_stdin(fixture.stateless_input_bytes),
        expected_public_values: fixture.stateless_output_bytes,
        metadata,
    };

    Ok(fixture.into_boxed())
}

fn zilkworm_input_from_fixture(
    fixture: StatelessValidationFixture,
) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    info!("Preparing Zilkworm stateless validator input for fixture {name}");
    let zilkworm_input = StatelessValidatorZilkwormInput::new(&stateless_input, success)
        .with_context(|| format!("building Zilkworm input for {name}"))?;
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(Box::new(ZilkwormGuestFixture {
        name,
        unified_rlp: zilkworm_input.unified_rlp,
        metadata,
    }))
}

#[derive(Debug)]
struct ZilkwormGuestFixture {
    name: String,
    unified_rlp: Vec<u8>,
    metadata: BlockMetadata,
}

impl GuestFixture for ZilkwormGuestFixture {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::to_value(&self.metadata).unwrap()
    }

    fn input(&self) -> Result<Input> {
        let bundle = stateless_validator_zilkworm::encode_unified_rlp_bundle(&[&self.unified_rlp]);
        Ok(Input::new().with_stdin(bundle))
    }

    fn expected_public_values(&self) -> Result<Vec<u8>> {
        let mut out = Vec::with_capacity(4 + 8);
        out.extend_from_slice(&1u32.to_le_bytes());
        out.extend_from_slice(&self.metadata.block_used_gas.to_le_bytes());
        Ok(out)
    }
}
