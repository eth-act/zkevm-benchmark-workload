//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{bail, Context, Result};
use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{Action, RunConfig, get_zkvm_instances, run_benchmark},
    stateless_executor, stateless_validator,
};

use clap::Parser;
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::ProverResourceType;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::cli::{Cli, GuestProgramCommand, Resource, StatelessExecutorClient, StatelessValidatorClient};

pub mod cli;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Validate that network proving is only used with SP1
    if matches!(cli.resource, Resource::Network) {
        if cli.zkvms.iter().any(|z| *z != zkVMKind::SP1) {
            bail!("Network proving is only supported for SP1. Use --zkvms sp1");
        }
    }

    let resource: ProverResourceType = cli.resource.clone().into();
    // validate that cluster proving is only used with sp1
    if matches!(cli.resource, Resource::Cluster) {
        if cli.zkvms.iter().any(|z| *z != zkVMKind::SP1) {
            bail!("Cluster proving is only supported with SP1 zkVMs");
        }
    }

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();
    info!(
        "Running benchmarks with resource={:?} and action={:?}",
        resource, action
    );

    let workspace_dir = workspace_root().join("ere-guests");
    match cli.guest_program {
        GuestProgramCommand::StatelessExecutor {
            input_folder,
            input_file,
            execution_client,
        } => {
            let input_display = input_file.as_ref().unwrap_or(&input_folder);
            info!(
                "Running stateless-executor benchmark for input: {}",
                input_display.display()
            );
            let el = execution_client.into();
            let guest_io = stateless_executor::stateless_executor_inputs_from(
                input_folder.as_path(),
                input_file.as_deref(),
                el,
            )?;
            let guest_relative = execution_client
                .guest_rel_path()
                .context("Failed to get guest relative path")?;
            let apply_patches = matches!(execution_client, StatelessExecutorClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                &guest_relative,
                resource,
                apply_patches,
            )?;
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
        GuestProgramCommand::StatelessValidator {
            input_folder,
            input_file,
            execution_client,
        } => {
            let input_display = input_file.as_ref().unwrap_or(&input_folder);
            info!(
                "Running stateless-validator benchmark for input: {}",
                input_display.display()
            );
            let el = execution_client.into();
            let guest_io = stateless_validator::stateless_validator_inputs_from(
                input_folder.as_path(),
                input_file.as_deref(),
                el,
            )?;
            let guest_relative = execution_client
                .guest_rel_path()
                .context("Failed to get guest relative path")?;
            let apply_patches = matches!(execution_client, StatelessValidatorClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                &guest_relative,
                resource,
                apply_patches,
            )?;
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
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("empty-program"),
                resource,
                true,
            )?;
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
            )?;
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("block-encoding-length"),
                resource,
                true,
            )?;
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

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
