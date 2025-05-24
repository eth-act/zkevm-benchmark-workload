//! Binary for benchmarking different Ere compatible zkVMs

use clap::{Parser, ValueEnum};
use std::{path::PathBuf, process::Command};
use strum::IntoEnumIterator;

// use ere_pico::{ErePico, PICO_TARGET};
use benchmark_runner::{Action, run_benchmark_ere};
use ere_openvm::{EreOpenVM, OPENVM_TARGET};
use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType};

#[derive(Parser)]
#[command(name = "zkvm-benchmarker")]
#[command(about = "Benchmark different Ere compatible zkVMs")]
#[command(version)]
struct Cli {
    /// zkVMs to benchmark (if none specified, runs all)
    #[arg(short, long, value_enum)]
    zkvm: Vec<zkVM>,

    /// Resource type for proving
    #[arg(short, long, value_enum, default_value = "cpu")]
    resource: Resource,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "execute")]
    action: BenchmarkAction,
}

#[derive(Clone, ValueEnum, strum::EnumIter)]
#[allow(non_camel_case_types)]
enum zkVM {
    Sp1,
    Risc0,
    Openvm,
    // Pico,
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
            Resource::Cpu => ProverResourceType::Cpu,
            Resource::Gpu => ProverResourceType::Gpu,
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Action::Execute,
            BenchmarkAction::Prove => Action::Prove,
        }
    }
}

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();

    // If no zkVM specified, run all
    let zkvms = if cli.zkvm.is_empty() {
        zkVM::iter().collect()
    } else {
        cli.zkvm
    };

    for zkvm in zkvms {
        match zkvm {
            zkVM::Sp1 => {
                run_cargo_patch_command("sp1")?;
                let sp1_zkvm = new_sp1_zkvm(resource)?;
                run_benchmark_ere("sp1", sp1_zkvm, action)?;
            }
            zkVM::Risc0 => {
                run_cargo_patch_command("risc0")?;
                let risc0_zkvm = new_risczero_zkvm(resource)?;
                run_benchmark_ere("risc0", risc0_zkvm, action)?;
            }
            zkVM::Openvm => {
                run_cargo_patch_command("openvm")?;
                let openvm_zkvm = new_openvm_zkvm(resource)?;
                run_benchmark_ere("openvm", openvm_zkvm, action)?;
            } // zkVM::Pico => {
              //     let pico_zkvm = new_pico_zkvm(resource)?;
              //     run_benchmark_ere("pico", pico_zkvm, action)?;
              // }
        }
    }

    Ok(())
}

fn new_sp1_zkvm(prover_resource: ProverResourceType) -> Result<EreSP1, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1");
    let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreSP1::new(program, prover_resource))
}

fn new_risczero_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreRisc0, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/risc0");
    let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreRisc0::new(program, prover_resource))
}

fn new_openvm_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreOpenVM, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/openvm");
    let program = OPENVM_TARGET::compile(&PathBuf::from(guest_dir))?;
    Ok(EreOpenVM::new(program, prover_resource))
}

// fn new_pico_zkvm(
//     prover_resource: ProverResourceType,
// ) -> Result<ErePico, Box<dyn std::error::Error>> {
//     let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/pico");
//     let program = PICO_TARGET::compile(&PathBuf::from(guest_dir))?;
//     Ok(ErePico::new(program, prover_resource))
// }

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(zkvm_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running cargo {}...", zkvm_name);

    let output = Command::new("cargo").arg(zkvm_name).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!(
            "cargo {} failed with exit code: {:?}",
            zkvm_name,
            output.status.code()
        );
        eprintln!("stdout: {}", stdout);
        eprintln!("stderr: {}", stderr);

        return Err(format!("cargo {} command failed", zkvm_name).into());
    }

    println!("cargo {} completed successfully", zkvm_name);
    Ok(())
}
