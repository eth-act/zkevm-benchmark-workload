//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[cfg(not(any(
    feature = "sp1",
    feature = "risc0",
    feature = "openvm",
    feature = "pico",
    feature = "zisk"
)))]
compile_error!("please enable one of the zkVM's using the appropriate feature flag");

use benchmark_runner::{
    Action, RunConfig, run_benchmark_empty_program, run_benchmark_rlp_encoding_length,
    run_benchmark_stateless_validator,
};
use clap::{Parser, Subcommand, ValueEnum};
use guest_libs::BincodeBlock;
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;
use witness_generator::BlockAndWitness;
use zkvm_interface::{Compiler, ProverResourceType, zkVM};

#[cfg(feature = "sp1")]
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};

#[cfg(feature = "risc0")]
use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};

#[cfg(feature = "openvm")]
use ere_openvm::{EreOpenVM, OPENVM_TARGET};

#[cfg(feature = "pico")]
use ere_pico::{ErePico, PICO_TARGET};

#[cfg(feature = "zisk")]
use ere_zisk::{EreZisk, RV64_IMA_ZISK_ZKVM_ELF};

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

    /// Block RLP length calculator
    RlpEncodingLength {
        /// Input folder for benchmark results
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,

        /// Number of times to loop the benchmark
        #[arg(long)]
        loop_count: u16,
    },
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

    let run_config = RunConfig {
        output_folder: cli.output_folder,
        action,
        force_rerun: cli.force_rerun,
    };

    let guest_programs_dir = workspace_root().join("ere-guests");
    match &cli.guest_program {
        GuestProgramCommand::StatelessValidator { input_folder } => {
            info!(
                "Running Ethereum Stateless Validator benchmarks, with corpus from: {}",
                input_folder.display()
            );
            let corpuses = WalkDir::new(input_folder)
                .min_depth(1)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_par_iter()
                .map(|entry| {
                    if entry.file_type().is_file() {
                        let content = std::fs::read(entry.path())?;
                        let blocks_and_witnesses =
                            serde_json::from_slice(&content).map_err(|e| {
                                anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                            })?;
                        Ok(blocks_and_witnesses)
                    } else {
                        anyhow::bail!("Invalid input folder structure: expected files only")
                    }
                })
                .collect::<Result<Vec<BlockAndWitness>, _>>()?;

            let zkvms_instances =
                get_zkvm_instances(&guest_programs_dir.join("stateless-validator"), resource)?;
            for instance in zkvms_instances {
                run_benchmark_stateless_validator(
                    &instance.name,
                    instance.instance,
                    &run_config,
                    &corpuses,
                )?;
            }
        }
        GuestProgramCommand::EmptyProgram => {
            info!("Running empty program benchmarks");
            let zkvms_instances =
                get_zkvm_instances(&guest_programs_dir.join("empty-program"), resource)?;
            for instance in zkvms_instances {
                run_benchmark_empty_program(&instance.name, instance.instance, &run_config)?;
            }
        }
        GuestProgramCommand::RlpEncodingLength {
            input_folder,
            loop_count,
        } => {
            info!("Running rlp encoding length benchmarks");
            let blocks = WalkDir::new(input_folder)
                .min_depth(1)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_par_iter()
                .map(|entry| {
                    if entry.file_type().is_file() {
                        let content = std::fs::read(entry.path())?;
                        let blocks_and_witnesses: BlockAndWitness =
                            serde_json::from_slice(&content).map_err(|e| {
                                anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                            })?;
                        Ok(BincodeBlock(blocks_and_witnesses.block_and_witness.block))
                    } else {
                        anyhow::bail!("Invalid input folder structure: expected files only")
                    }
                })
                .collect::<Result<Vec<BincodeBlock>, _>>()?;

            let zkvms_instances =
                get_zkvm_instances(&guest_programs_dir.join("rlp-encoding-length"), resource)?;
            for instance in zkvms_instances {
                run_benchmark_rlp_encoding_length(
                    &instance.name,
                    instance.instance,
                    &run_config,
                    &blocks,
                    *loop_count,
                )?;
            }
        }
    }

    Ok(())
}

#[allow(non_camel_case_types)]
struct zkVMInstance {
    pub name: String,
    pub instance: Box<dyn zkVM + Sync>,
}

fn get_zkvm_instances(
    guest_program_folder: &Path,
    resource: ProverResourceType,
) -> Result<Vec<zkVMInstance>, Box<dyn std::error::Error>> {
    let mut name_zkvms: Vec<zkVMInstance> = Default::default();
    #[allow(clippy::redundant_clone)]
    {
        #[cfg(feature = "sp1")]
        {
            run_cargo_patch_command("sp1", Some(guest_program_folder))?;
            let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&guest_program_folder.join("sp1"))?;
            let zkvm = EreSP1::new(program, resource.clone());
            name_zkvms.push(zkVMInstance {
                name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
                instance: Box::new(zkvm),
            });
        }

        #[cfg(feature = "zisk")]
        {
            run_cargo_patch_command("zisk", None)?;
            let program = RV64_IMA_ZISK_ZKVM_ELF::compile(&guest_program_folder.join("zisk"))?;
            let zkvm = EreZisk::new(program, resource.clone());
            name_zkvms.push(zkVMInstance {
                name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
                instance: Box::new(zkvm),
            });
        }

        #[cfg(feature = "risc0")]
        {
            run_cargo_patch_command("risc0", None)?;
            let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&guest_program_folder.join("risc0"))?;
            let zkvm = EreRisc0::new(program, resource.clone());
            name_zkvms.push(zkVMInstance {
                name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
                instance: Box::new(zkvm),
            });
        }

        #[cfg(feature = "openvm")]
        {
            run_cargo_patch_command("openvm", None)?;
            let program = OPENVM_TARGET::compile(&guest_program_folder.join("openvm"))?;
            let zkvm = EreOpenVM::new(program, resource.clone());
            name_zkvms.push(zkVMInstance {
                name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
                instance: Box::new(zkvm),
            });
        }

        #[cfg(feature = "pico")]
        {
            run_cargo_patch_command("pico", None)?;
            let program = PICO_TARGET::compile(&guest_program_folder.join("pico"))?;
            let zkvm = ErePico::new(program, resource.clone());
            name_zkvms.push(zkVMInstance {
                name: zkvm_fullname(zkvm.name(), zkvm.sdk_version()),
                instance: Box::new(zkvm),
            });
        }
    }
    Ok(name_zkvms)
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(
    zkvm_name: &str,
    guest_program_type_folder: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running cargo {}...", zkvm_name);

    let mut cmd = Command::new("cargo");
    cmd.arg(zkvm_name);
    if let Some(guest_program_type_folder) = guest_program_type_folder {
        cmd.arg("--manifest-folder")
            .arg(guest_program_type_folder.join(zkvm_name));
    }
    let output = cmd.output()?;

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

fn zkvm_fullname(zkvm_name: &str, zkvm_version: &str) -> String {
    format!("{zkvm_name}-v{zkvm_version}")
}

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
