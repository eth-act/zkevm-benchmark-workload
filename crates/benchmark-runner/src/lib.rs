use anyhow::*;
use rayon::prelude::*;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use witness_generator::{generate_stateless_witness, BlocksAndWitnesses};
use zkevm_metrics::WorkloadMetrics;
use zkvm_interface::{zkVM, Compiler, Input, ProverResourceType};

#[deprecated(note = "this function is being phased out, use run_benchmark_ere")]
pub fn run_benchmark<F>(elf_path: &'static [u8], metrics_path_prefix: &str, zkvm_executor: F)
where
    F: Fn(&BlocksAndWitnesses, &'static [u8]) -> Vec<WorkloadMetrics> + Send + Sync,
{
    let generated_corpuses = generate_stateless_witness::generate();

    generated_corpuses.into_par_iter().for_each(|bw| {
        println!("{} (num_blocks={})", bw.name, bw.blocks_and_witnesses.len());

        let reports = zkvm_executor(&bw, elf_path);

        WorkloadMetrics::to_path(
            format!(
                "{}/{}/{}/{}.json",
                env!("CARGO_WORKSPACE_DIR"),
                "zkevm-metrics",
                metrics_path_prefix,
                bw.name
            ),
            &reports,
        )
        .unwrap();

        println!(
            "Finished processing and saved metrics for corpus: {}. Number of reports: {}",
            bw.name,
            reports.len()
        );
        // dbg!(&reports);
    });
}

pub fn run_benchmark_ere<C, V>(host_name: &str, guest_dir: &str) -> Result<()>
where
    C: Compiler + Send + Sync,
    C::Error: std::error::Error + Send + Sync + 'static,
    V: zkVM<C> + Sync,
    V::Error: std::error::Error + Send + Sync + 'static,
{
    println!("Benchmarking `{}`…", host_name);

    // Compile program and create proving/verification keys
    let prover_resource = ProverResourceType::Cpu;
    let program = C::compile(&PathBuf::from(guest_dir))?;
    let zkvm_instance = V::new(program, prover_resource);
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
                proving_time_ms: 0,
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
