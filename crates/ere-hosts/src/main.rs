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

use benchmark_runner::{Action, RunConfig, run_benchmark_ere};
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{path::PathBuf, process::Command};
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

    /// Input folder for benchmark results
    #[arg(short, long, default_value = "zkevm-fixtures-input")]
    input_folder: PathBuf,

    /// Output folder for benchmark results
    #[arg(short, long, default_value = "zkevm-metrics")]
    output_folder: PathBuf,
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

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();

    info!("Loading corpuses from: {}", cli.input_folder.display());
    let corpuses = WalkDir::new(&cli.input_folder)
        .min_depth(1)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_par_iter()
        .map(|entry| {
            if entry.file_type().is_file() {
                let content = std::fs::read(entry.path())?;
                let blocks_and_witnesses = serde_json::from_slice(&content).map_err(|e| {
                    anyhow::anyhow!("Failed to parse {}: {}", entry.path().display(), e)
                })?;
                Ok(blocks_and_witnesses)
            } else {
                anyhow::bail!("Invalid input folder structure: expected files only")
            }
        })
        .collect::<Result<Vec<BlockAndWitness>, _>>()?;

    #[allow(clippy::redundant_clone)]
    {
        let run_config = RunConfig {
            output_folder: cli.output_folder,
            action,
            force_rerun: cli.force_rerun,
        };

        info!("Running benchmarks with resource: {:?}", resource);
        #[cfg(feature = "sp1")]
        {
            run_cargo_patch_command("sp1")?;
            let zkvm = new_sp1_zkvm(resource.clone())?;
            let fullname = zkvm_fullname(EreSP1::name(), EreSP1::sdk_version());
            run_benchmark_ere(&fullname, zkvm, &run_config, &corpuses)?;
        }

        #[cfg(feature = "zisk")]
        {
            run_cargo_patch_command("zisk")?;
            let zisk_zkvm = new_zisk_zkvm(resource.clone())?;
            let fullname = zkvm_fullname(EreZisk::name(), EreZisk::sdk_version());
            run_benchmark_ere(&fullname, zisk_zkvm, &run_config, &corpuses)?;
        }

        #[cfg(feature = "risc0")]
        {
            run_cargo_patch_command("risc0")?;
            let risc0_zkvm = new_risczero_zkvm(resource.clone())?;
            let fullname = zkvm_fullname(EreRisc0::name(), EreRisc0::sdk_version());
            run_benchmark_ere(&fullname, risc0_zkvm, &run_config, &corpuses)?;
        }

        #[cfg(feature = "openvm")]
        {
            run_cargo_patch_command("openvm")?;
            let openvm_zkvm = new_openvm_zkvm(resource.clone())?;
            let fullname = zkvm_fullname(EreOpenVM::name(), EreOpenVM::sdk_version());
            run_benchmark_ere(&fullname, openvm_zkvm, &run_config, &corpuses)?;
        }

        #[cfg(feature = "pico")]
        {
            run_cargo_patch_command("pico")?;
            let pico_zkvm = new_pico_zkvm(resource)?;
            let fullname = zkvm_fullname(ErePico::name(), ErePico::sdk_version());
            run_benchmark_ere(&fullname, pico_zkvm, &run_config, &corpuses)?;
        }

        Ok(())
    }
}

#[cfg(feature = "sp1")]
fn new_sp1_zkvm(prover_resource: ProverResourceType) -> Result<EreSP1, Box<dyn std::error::Error>> {
    let guest_dir = workspace_root().join("ere-guests/sp1");
    let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&guest_dir)?;
    Ok(EreSP1::new(program, prover_resource))
}

#[cfg(feature = "risc0")]
fn new_risczero_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreRisc0, Box<dyn std::error::Error>> {
    let guest_dir = workspace_root().join("ere-guests/risc0");
    let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&guest_dir)?;
    Ok(EreRisc0::new(program, prover_resource))
}

#[cfg(feature = "zisk")]
fn new_zisk_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreZisk, Box<dyn std::error::Error>> {
    let guest_dir = workspace_root().join("ere-guests/zisk");
    let program = RV64_IMA_ZISK_ZKVM_ELF::compile(&guest_dir)?;
    Ok(EreZisk::new(program, prover_resource))
}

#[cfg(feature = "openvm")]
fn new_openvm_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreOpenVM, Box<dyn std::error::Error>> {
    let guest_dir = workspace_root().join("ere-guests/openvm");
    let program = OPENVM_TARGET::compile(&guest_dir)?;
    Ok(EreOpenVM::new(program, prover_resource))
}

#[cfg(feature = "pico")]
fn new_pico_zkvm(
    prover_resource: ProverResourceType,
) -> Result<ErePico, Box<dyn std::error::Error>> {
    let guest_dir = workspace_root().join("ere-guests/pico");
    let program = PICO_TARGET::compile(&guest_dir)?;
    Ok(ErePico::new(program, prover_resource))
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(zkvm_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running cargo {}...", zkvm_name);

    let output = Command::new("cargo").arg(zkvm_name).output()?;

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
