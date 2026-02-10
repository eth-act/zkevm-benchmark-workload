//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result, bail};
use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{
        Action, ProfileConfig, RunConfig, get_el_zkvm_instances, get_guest_zkvm_instances,
        run_benchmark,
    },
    stateless_validator::{self},
    verification::{download_and_extract_proofs, resolve_extracted_root, run_verify_from_disk},
};
use ere_dockerized::zkVMKind;

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

    if cli.zisk_profile {
        if !matches!(cli.action, cli::BenchmarkAction::Execute) {
            bail!(
                "--zisk-profile requires --action execute, but got {:?}",
                cli.action
            );
        }
        if cli.zkvms.len() != 1 || cli.zkvms[0] != zkVMKind::Zisk {
            let zkvm_names: Vec<_> = cli.zkvms.iter().map(|z| z.as_str()).collect();
            bail!(
                "--zisk-profile requires --zkvms zisk only, but got: {}",
                zkvm_names.join(", ")
            );
        }
    }

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();
    info!(
        "Running benchmarks with resource={:?} and action={:?}",
        resource, action
    );

    let zisk_profile_config = cli
        .zisk_profile
        .then(|| ProfileConfig::new(cli.zisk_profile_output.clone()));

    // Validate: --save-proofs is only valid with --action prove
    if cli.save_proofs.is_some() && !matches!(action, Action::Prove) {
        anyhow::bail!("--save-proofs is only valid with --action prove");
    }

    // Validate: --proofs-url is only valid with --action verify
    if cli.proofs_url.is_some() && !matches!(action, Action::Verify) {
        anyhow::bail!("--proofs-url is only valid with --action verify");
    }

    // Resolve proofs source: download from URL or use local folder.
    // _proofs_tmpdir must live until verification completes (drop = cleanup).
    let (_proofs_tmpdir, proofs_folder) = if let Some(ref url) = cli.proofs_url {
        let tmp = download_and_extract_proofs(url).await?;
        let resolved = resolve_extracted_root(tmp.path())?;
        (Some(tmp), resolved)
    } else {
        (None, cli.proofs_folder)
    };
    let bin_path = cli.bin_path.as_deref();
    let config_base = RunConfig {
        output_folder: cli.output_folder,
        sub_folder: None,
        action,
        force_rerun: cli.force_rerun,
        dump_inputs_folder: cli.dump_inputs,
        zisk_profile_config,
        save_proofs_folder: cli.save_proofs,
    };

    match cli.guest_program {
        GuestProgramCommand::StatelessValidator {
            input_folder,
            execution_client,
        } => {
            let el: stateless_validator::ExecutionClient = execution_client.into();

            let el_name = el.as_ref().to_lowercase();
            let el_str = format!("{}-{}", el_name, el.version());
            let zkvms = get_el_zkvm_instances(&el_name, &cli.zkvms, resource, bin_path)
                .await
                .context("Failed to get EL zkvm instances")?;

            let config = RunConfig {
                sub_folder: Some(el_str),
                ..config_base
            };

            match action {
                Action::Verify => {
                    for instance in &zkvms {
                        run_verify_from_disk(&instance.zkvm, &config, &proofs_folder)?;
                    }
                }
                _ => {
                    info!(
                        "Running stateless-validator benchmark for input folder: {}",
                        input_folder.display()
                    );
                    let guest_io =
                        stateless_validator::stateless_validator_inputs(input_folder.as_path(), el)
                            .context("Failed to get stateless validator inputs")?;

                    for zkvm in &zkvms {
                        run_benchmark(zkvm, &config, &guest_io)?;
                    }
                }
            }
        }
        GuestProgramCommand::EmptyProgram => {
            info!("Running empty-program benchmarks");
            let zkvms = get_guest_zkvm_instances("empty", &cli.zkvms, resource, bin_path)
                .await
                .context("Failed to get guest zkvm instances")?;

            match action {
                Action::Verify => {
                    for instance in &zkvms {
                        run_verify_from_disk(&instance.zkvm, &config_base, &proofs_folder)?;
                    }
                }
                _ => {
                    let guest_io = empty_program::empty_program_input()
                        .context("Failed to create empty program input")?;
                    for zkvm in zkvms {
                        run_benchmark(&zkvm, &config_base, [&guest_io])?;
                    }
                }
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
            let zkvms =
                get_guest_zkvm_instances("block-encoding-length", &cli.zkvms, resource, bin_path)
                    .await
                    .context("Failed to get block encoding length zkvm instances")?;

            match action {
                Action::Verify => {
                    for instance in &zkvms {
                        run_verify_from_disk(&instance.zkvm, &config_base, &proofs_folder)?;
                    }
                }
                _ => {
                    let guest_io = block_encoding_length_program::block_encoding_length_inputs(
                        input_folder.as_path(),
                        loop_count,
                        format.into(),
                    )
                    .context("Failed to get block encoding length inputs")?;
                    for zkvm in zkvms {
                        run_benchmark(&zkvm, &config_base, &guest_io)?;
                    }
                }
            }
        }
    }

    Ok(())
}
