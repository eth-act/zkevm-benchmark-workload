//! Binary for benchmarking different Ere compatible zkVMs

use std::path::PathBuf;

use benchmark_runner::run_benchmark_ere;
use ere_sp1::EreSP1;
use zkvm_interface::{Compiler, ProverResourceType};

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sp1_zkvm = new_sp1_zkvm()?;
    run_benchmark_ere("sp1", sp1_zkvm)?;

    // TODO: Add more backends
    Ok(())
}

fn new_sp1_zkvm() -> Result<EreSP1, Box<dyn std::error::Error>> {
    let prover_resource = ProverResourceType::Cpu;
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1");
    let program = ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(ere_sp1::EreSP1::new(program, prover_resource))
}
