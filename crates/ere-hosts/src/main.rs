//! Binary for benchmarking different Ere compatible zkVMs

use benchmark_runner::run_benchmark_ere;

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SP1 zkVM
    run_benchmark_ere::<ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF, ere_sp1::EreSP1>(
        "sp1",
        concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1"),
    )?;

    // TODO: Add more backends
    Ok(())
}
