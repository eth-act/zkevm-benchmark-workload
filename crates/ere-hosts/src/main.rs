//! Binary for benchmarking different Ere compatible zkVMs

use std::path::PathBuf;

use benchmark_runner::{Action, run_benchmark_ere};
use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType};

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resource = ProverResourceType::Cpu;
    let sp1_zkvm = new_sp1_zkvm(resource)?;
    let action = Action::Execute;
    run_benchmark_ere("sp1", sp1_zkvm, action)?;

    let resource = ProverResourceType::Cpu;
    let risc0_zkvm = new_risczero_zkvm(resource)?;
    let action = Action::Execute;
    run_benchmark_ere("risc0", risc0_zkvm, action)?;

    // TODO: Add more backends
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
