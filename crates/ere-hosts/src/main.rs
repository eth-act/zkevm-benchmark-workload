//! Binary for benchmarking different Ere compatible zkVMs

use std::{path::PathBuf, process::Command};

// use ere_pico::{ErePico, PICO_TARGET};

use benchmark_runner::{Action, run_benchmark_ere};
use ere_openvm::{EreOpenVM, OPENVM_TARGET};
use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};

use zkvm_interface::{Compiler, ProverResourceType};

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cargo_patch_command("sp1")?;
    let resource = ProverResourceType::Cpu;
    let sp1_zkvm = new_sp1_zkvm(resource)?;
    let action = Action::Execute;
    run_benchmark_ere("sp1", sp1_zkvm, action)?;

    run_cargo_patch_command("risc0")?;
    let resource = ProverResourceType::Cpu;
    let risc0_zkvm = new_risczero_zkvm(resource)?;
    let action = Action::Execute;
    run_benchmark_ere("risc0", risc0_zkvm, action)?;

    // run_cargo_patch_command("openvm")?;
    let resource = ProverResourceType::Cpu;
    let openvm_zkvm = new_openvm_zkvm(resource)?;
    let action = Action::Execute;
    run_benchmark_ere("openvm", openvm_zkvm, action)?;

    // TODO: Symbol conflict with Risc0, See #42
    // let resource = ProverResourceType::Cpu;
    // let pico_zkvm = new_pico_zkvm(resource)?;
    // let action = Action::Execute;
    // run_benchmark_ere("pico", pico_zkvm, action)?;

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
