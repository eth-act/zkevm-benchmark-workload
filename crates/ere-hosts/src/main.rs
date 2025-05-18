//! Binary for benchmarking different Ere compatible zkVMs

use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use witness_generator::generate_stateless_witness;
use zkevm_metrics::WorkloadMetrics;
use zkvm_interface::{Compiler, Input, zkVM};

/// Main entry point for the host benchmarker
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    // SP1 zkVM
    benchmark::<ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF, ere_sp1::EreSP1>(
        "sp1",
        concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1"),
    )?;

    // TODO: Add more backends
    Ok(())
}

// TODO: Eventually move this into benchmark_runner
fn benchmark<C, V>(host_name: &str, guest_dir: &str) -> Result<()>
where
    C: Compiler + Send + Sync,
    C::Error: std::error::Error + Send + Sync + 'static,
    V: zkVM<C> + Sync,
    V::Error: std::error::Error + Send + Sync + 'static,
{
    println!("Benchmarking `{}`…", host_name);

    // Compile program and create proving/verification keys
    let program = C::compile(&PathBuf::from(guest_dir))?;
    let zkvm_instance = V::new(program);
    let zkvm_ref = Arc::new(&zkvm_instance);

    let corpuses = generate_stateless_witness::generate();

    // Iterate through test corpus and generate reports
    // TODO: note that when proving, processing these in parallel will likely skew the results
    corpuses.par_iter().try_for_each(|bw| -> Result<()> {
        println!(" {} ({} blocks)", bw.name, bw.blocks_and_witnesses.len());

        let mut reports = Vec::new();
        for ci in &bw.blocks_and_witnesses {
            let mut stdin = Input::new();
            stdin.write(ci)?;
            stdin.write(&bw.network)?;

            let report = zkvm_ref.execute(&stdin)?;
            let region_cycles: HashMap<_, _> = report.region_cycles.into_iter().collect();

            reports.push(WorkloadMetrics {
                name: format!("{}-{}", bw.name, ci.block.number),
                total_num_cycles: report.total_num_cycles,
                region_cycles,
            });
        }

        let out_path = format!(
            "{}/zkevm-metrics/{}/{}.json",
            env!("CARGO_WORKSPACE_DIR"),
            host_name,
            bw.name
        );
        WorkloadMetrics::to_path(out_path, &reports)?;
        println!("wrote {} reports", reports.len());

        Ok(())
    })?;

    Ok(())
}
