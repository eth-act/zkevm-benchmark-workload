//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result};
use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{Action, RunConfig, get_el_zkvm_instances, get_guest_zkvm_instances, run_benchmark},
    stateless_validator::{self},
};

use clap::Parser;
use ere_zkvm_interface::ProverResourceType;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::cli::{Cli, GuestProgramCommand};

pub mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();
    info!(
        "Running benchmarks with resource={:?} and action={:?}",
        resource, action
    );

    match cli.guest_program {
        GuestProgramCommand::StatelessValidator {
            input_folder,
            execution_client,
        } => {
            info!(
                "Running stateless-validator benchmark for input folder: {}",
                input_folder.display()
            );
            let el = execution_client.into();
            let guest_io =
                stateless_validator::stateless_validator_inputs(input_folder.as_path(), el)
                    .context("Failed to get stateless validator inputs")?;

            let zkvms = get_el_zkvm_instances(execution_client.into(), &cli.zkvms, resource)
                .await
                .context("Failed to get EL zkvm instances")?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: Some(el.as_ref().to_lowercase()),
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, &guest_io)?;
            }
        }
        GuestProgramCommand::EmptyProgram => {
            info!("Running empty-program benchmarks");
            let guest_io = empty_program::empty_program_input()
                .context("Failed to create empty program input")?;
            let zkvms = get_guest_zkvm_instances("empty-program", &cli.zkvms, resource).await?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: None,
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, [&guest_io])?;
            }
        }
        GuestProgramCommand::BlockEncodingLength {
            input_folder,
            loop_count,
            format,
        } => {
            info!(
                "Running {:?}-encoding-length benchmarks for input folder {} and loop count {}",
                format,
                input_folder.display(),
                loop_count
            );
            let guest_io = block_encoding_length_program::block_encoding_length_inputs(
                input_folder.as_path(),
                loop_count,
                format.into(),
            )
            .context("Failed to get block encoding length inputs")?;
            let zkvms = get_guest_zkvm_instances("block-encoding-length", &cli.zkvms, resource)
                .await
                .context("Failed to get block encoding length zkvm instances")?;
            let config = RunConfig {
                output_folder: cli.output_folder,
                sub_folder: None,
                action,
                force_rerun: cli.force_rerun,
                dump_inputs_folder: cli.dump_inputs.clone(),
            };
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, &guest_io)?;
            }
        }
    }

    Ok(())
}
