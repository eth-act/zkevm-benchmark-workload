//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{path::PathBuf, process::Command};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;
use walkdir::WalkDir;

use witness_generator::BlockAndWitness;

use benchmark_runner::{Action, RunConfig, run_benchmark_ere};

use zkvm_interface::{Compiler, ProverResourceType};

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

    #[allow(unused_assignments)]
    {
        // Set to true once a zkvm has ran
        let mut ran_any = false;

        let run_config = RunConfig {
            output_folder: cli.output_folder,
            action,
            force_rerun: cli.force_rerun,
        };

        info!("Running benchmarks with resource: {:?}", resource);
        #[cfg(feature = "sp1")]
        {
            run_cargo_patch_command("sp1")?;
            let sp1_zkvm = new_sp1_zkvm(resource.clone())?;
            run_benchmark_ere("sp1", sp1_zkvm, &run_config, &corpuses)?;
            ran_any = true;
        }

        #[cfg(feature = "zisk")]
        {
            run_cargo_patch_command("zisk")?;
            let zisk_zkvm = new_zisk_zkvm(resource.clone())?;
            run_benchmark_ere("zisk", zisk_zkvm, &run_config, &corpuses)?;
            ran_any = true;
        }

        #[cfg(feature = "risc0")]
        {
            run_cargo_patch_command("risc0")?;
            let risc0_zkvm = new_risczero_zkvm(resource.clone())?;
            run_benchmark_ere("risc0", risc0_zkvm, &run_config, &corpuses)?;
            ran_any = true;
        }

        #[cfg(feature = "openvm")]
        {
            run_cargo_patch_command("openvm")?;
            let openvm_zkvm = new_openvm_zkvm(resource.clone())?;
            run_benchmark_ere("openvm", openvm_zkvm, &run_config, &corpuses)?;
            ran_any = true;
        }

        #[cfg(feature = "pico")]
        {
            run_cargo_patch_command("pico")?;
            let pico_zkvm = new_pico_zkvm(resource)?;
            run_benchmark_ere("pico", pico_zkvm, &run_config, &corpuses)?;
            ran_any = true;
        }

        if ran_any {
            Ok(())
        } else {
            Err("please enable one of the zkVM's using the appropriate feature flag".into())
        }
    }
}

#[cfg(feature = "sp1")]
fn new_sp1_zkvm(prover_resource: ProverResourceType) -> Result<EreSP1, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1");
    let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreSP1::new(program, prover_resource))
}

#[cfg(feature = "risc0")]
fn new_risczero_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreRisc0, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/risc0");
    let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreRisc0::new(program, prover_resource))
}

#[cfg(feature = "zisk")]
fn new_zisk_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreZisk, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/zisk");
    let program = RV64_IMA_ZISK_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreZisk::new(program, prover_resource))
}

#[cfg(feature = "openvm")]
fn new_openvm_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreOpenVM, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/openvm");
    let program = OPENVM_TARGET::compile(&PathBuf::from(guest_dir))?;
    Ok(EreOpenVM::new(program, prover_resource))
}

#[cfg(feature = "pico")]
fn new_pico_zkvm(
    prover_resource: ProverResourceType,
) -> Result<ErePico, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/pico");
    let program = PICO_TARGET::compile(&PathBuf::from(guest_dir))?;
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
