//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use benchmark_runner::{Action, RunConfig, guest_programs, run_benchmark};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::{EreDockerizedCompiler, EreDockerizedzkVM, ErezkVM};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use zkvm_interface::{Compiler, ProverResourceType};

#[derive(Parser)]
#[command(name = "zkvm-benchmarker")]
#[command(about = "Benchmark different Ere compatible zkVMs")]
#[command(version)]
struct Cli {
    /// Resource type for proving
    #[arg(short, long, value_enum, default_value = "cpu")]
    resource: Resource,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "execute")]
    action: BenchmarkAction,

    /// zkVM instances to benchmark
    #[arg(long, required(true), value_parser = <ErezkVM as std::str::FromStr>::from_str)]
    zkvms: Vec<ErezkVM>,

    /// Rerun the benchmarks even if the output folder already contains results
    #[arg(long, default_value_t = false)]
    force_rerun: bool,

    /// Guest program to benchmark
    #[command(subcommand)]
    guest_program: GuestProgramCommand,

    /// Output folder for benchmark results
    #[arg(short, long, default_value = "zkevm-metrics")]
    output_folder: PathBuf,
}

#[derive(Subcommand, Clone, Debug)]
enum GuestProgramCommand {
    /// Ethereum Stateless Validator
    StatelessValidator {
        /// Input folder for benchmark results
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,
    },
    /// Empty program
    EmptyProgram,

    /// Block encoding length
    BlockEncodingLength {
        /// Input folder for benchmark results
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,

        /// Number of times to loop the benchmark
        #[arg(long)]
        loop_count: u16,

        /// Encoding format
        #[arg(short, long, value_enum)]
        format: BlockEncodingFormat,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum BlockEncodingFormat {
    Rlp,
    Ssz,
}

#[derive(Clone, ValueEnum)]
enum Resource {
    Cpu,
    Gpu,
}

#[derive(Clone, ValueEnum)]
enum BenchmarkAction {
    Execute,
    Prove,
}

impl From<Resource> for ProverResourceType {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Cpu => Self::Cpu,
            Resource::Gpu => Self::Gpu,
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Self::Execute,
            BenchmarkAction::Prove => Self::Prove,
        }
    }
}

impl From<BlockEncodingFormat> for guest_programs::BlockEncodingFormat {
    fn from(format: BlockEncodingFormat) -> Self {
        match format {
            BlockEncodingFormat::Rlp => Self::Rlp,
            BlockEncodingFormat::Ssz => Self::Ssz,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let config = RunConfig {
        output_folder: cli.output_folder,
        action,
        force_rerun: cli.force_rerun,
    };

    let workspace_dir = workspace_root().join("ere-guests");
    match &cli.guest_program {
        GuestProgramCommand::StatelessValidator { input_folder } => {
            info!(
                "Running stateless-validator benchmark for input folder: {}",
                input_folder.display()
            );
            let inputs = guest_programs::stateless_validator_inputs(input_folder.as_path())?;
            for zkvm in cli.zkvms {
                let zkvm = get_zkvm_instance(
                    &zkvm,
                    &workspace_dir,
                    Path::new("stateless-validator"),
                    resource.clone(),
                )?;
                run_benchmark(&zkvm, &config, inputs.clone())?;
            }
        }
        GuestProgramCommand::EmptyProgram => {
            info!("Running empty-program benchmarks");
            let input = guest_programs::empty_program_inputs();

            for zkvm in cli.zkvms {
                let zkvm = get_zkvm_instance(
                    &zkvm,
                    &workspace_dir,
                    Path::new("empty-program"),
                    resource.clone(),
                )?;
                run_benchmark(&zkvm, &config, vec![input.clone()])?;
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
            let inputs = guest_programs::block_encoding_length_inputs(
                input_folder.as_path(),
                *loop_count,
                format.clone().into(),
            )?;

            for zkvm in cli.zkvms {
                let zkvm = get_zkvm_instance(
                    &zkvm,
                    &workspace_dir,
                    Path::new("block-encoding-length"),
                    resource.clone(),
                )?;
                run_benchmark(&zkvm, &config, inputs.clone())?;
            }
        }
    }

    Ok(())
}

fn get_zkvm_instance(
    zkvm: &ErezkVM,
    workspace_dir: &Path,
    guest_relative: &Path,
    resource: ProverResourceType,
) -> Result<EreDockerizedzkVM, Box<dyn std::error::Error>> {
    run_cargo_patch_command(zkvm.as_str(), workspace_dir)?;
    let program = EreDockerizedCompiler::new(*zkvm, workspace_dir)
        .compile(&workspace_dir.join(guest_relative).join(zkvm.as_str()))?;
    Ok(EreDockerizedzkVM::new(*zkvm, program, resource.clone())?)
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(
    zkvm_name: &str,
    workspace_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running cargo {}...", zkvm_name);

    let output = Command::new("cargo")
        .arg(zkvm_name)
        .arg("--manifest-folder")
        .arg(workspace_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        error!(
            "cargo {} failed with exit code: {:?}",
            zkvm_name,
            output.status.code()
        );
        error!("stdout: {}", stdout);
        error!("stderr: {}", stderr);

        return Err(format!("cargo {zkvm_name} command failed").into());
    }

    info!("cargo {zkvm_name} completed successfully");
    Ok(())
}

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
