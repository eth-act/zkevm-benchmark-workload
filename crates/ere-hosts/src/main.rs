//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result};
use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{Action, RunConfig, get_zkvm_instances, run_benchmark},
    stateless_validator::{self},
};

use clap::Parser;
use ere_zkvm_interface::ProverResourceType;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::cli::{Cli, ExecutionClient, GuestProgramCommand};

pub mod cli;

fn main() -> Result<()> {
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

    let workspace_dir = workspace_root().join("ere-guests");
    match cli.guest_program {
        GuestProgramCommand::StatelessValidator {
            input_folder,
            execution_client,
            block_body_kzg_commit,
        } => {
            info!(
                "Running stateless-validator benchmark for input folder: {}",
                input_folder.display()
            );
            let el = execution_client.into();
            let guest_io = stateless_validator::stateless_validator_inputs(
                input_folder.as_path(),
                el,
                block_body_kzg_commit.into(),
            )?;
            let guest_relative = execution_client
                .guest_rel_path()
                .context("Failed to get guest relative path")?;
            let apply_patches = matches!(execution_client, ExecutionClient::Reth);
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
                run_benchmark(&zkvm, &config, guest_io.clone())?;
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
                run_benchmark(&zkvm, &config, vec![guest_io.clone()])?;
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
                run_benchmark(&zkvm, &config, guest_io.clone())?;
            }
        }
        GuestProgramCommand::AnalyzeCompression { input_folder } => {
            info!(
                "Analyzing compression for fixtures in: {}",
                input_folder.display()
            );
            let mut results = stateless_validator::analyze_compression(&input_folder)?;

            results.sort_by(|a, b| b.blob_savings.cmp(&a.blob_savings));

            println!(
                "\n{:<50} {:>12} {:>12} {:>8} {:>10} {:>12} {:>8}",
                "Fixture", "Raw Size", "Compressed", "Ratio", "Raw Blobs", "Snappy Blobs", "Saved"
            );
            println!("{}", "-".repeat(114));

            // Accumulators for totals
            let mut total_raw_size: usize = 0;
            let mut total_compressed_size: usize = 0;
            let mut total_raw_blobs: usize = 0;
            let mut total_compressed_blobs: usize = 0;

            for result in &results {
                println!(
                    "{:<50} {:>12} {:>12} {:>7.1}% {:>10} {:>12} {:>8}",
                    truncate_name(&result.name, 50),
                    format_bytes(result.raw_size),
                    format_bytes(result.compressed_size),
                    result.compression_ratio * 100.0,
                    result.raw_blobs,
                    result.compressed_blobs,
                    result.blob_savings
                );
                total_raw_size += result.raw_size;
                total_compressed_size += result.compressed_size;
                total_raw_blobs += result.raw_blobs;
                total_compressed_blobs += result.compressed_blobs;
            }

            // Print totals
            println!("{}", "-".repeat(114));
            let total_ratio = if total_raw_size > 0 {
                total_compressed_size as f64 / total_raw_size as f64
            } else {
                0.0
            };
            let total_savings = total_raw_blobs as i32 - total_compressed_blobs as i32;
            println!(
                "{:<50} {:>12} {:>12} {:>7.1}% {:>10} {:>12} {:>8}",
                format!("TOTAL ({} fixtures)", results.len()),
                format_bytes(total_raw_size),
                format_bytes(total_compressed_size),
                total_ratio * 100.0,
                total_raw_blobs,
                total_compressed_blobs,
                total_savings
            );
            println!();
        }
    }

    Ok(())
}

fn format_bytes(bytes: usize) -> String {
    let s = bytes.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        name.to_string()
    } else {
        format!("...{}", &name[name.len() - max_len + 3..])
    }
}

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
