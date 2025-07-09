//! Benchmark runner for zkVM workloads

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use rayon::prelude::*;
use std::{any::Any, panic, path::PathBuf, sync::Arc};
use tracing::info;
use witness_generator::BlockAndWitness;
use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};
use zkvm_interface::{zkVM, Input};

/// Holds the configuration for running benchmarks
#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Output folder where benchmark results will be stored
    pub output_folder: PathBuf,
    /// Action to perform: either proving or executing
    pub action: Action,
    /// Force rerun benchmarks even if output files already exist
    pub force_rerun: bool,
}

/// Action specifies whether we should prove or execute
#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Generate a proof for the zkVM execution
    Prove,
    /// Only execute the zkVM without proving
    Execute,
}

/// Runs the benchmark for a given zkVM instance and corpus of blocks and witnesses
pub fn run_benchmark_ere<V>(
    host_name: &str,
    zkvm_instance: V,
    run_config: &RunConfig,
    corpuses: &[BlockAndWitness],
) -> anyhow::Result<()>
where
    V: zkVM + Sync,
{
    HardwareInfo::detect().to_path(run_config.output_folder.join("hardware.json"))?;

    info!("Benchmarking `{}`…", host_name);
    let zkvm_ref = Arc::new(&zkvm_instance);

    match run_config.action {
        Action::Execute => {
            // Use parallel iteration for execution
            corpuses
                .into_par_iter()
                .try_for_each(|bw| process_corpus(bw, zkvm_ref.clone(), host_name, run_config))?;
        }
        Action::Prove => {
            // Use sequential iteration for proving
            corpuses
                .iter()
                .try_for_each(|bw| process_corpus(bw, zkvm_ref.clone(), host_name, run_config))?;
        }
    }
    Ok(())
}

fn process_corpus<V>(
    bw: &BlockAndWitness,
    zkvm_ref: Arc<V>,
    host_name: &str,
    run_config: &RunConfig,
) -> anyhow::Result<()>
where
    V: zkVM + Sync,
{
    let out_path = run_config
        .output_folder
        .join(format!("{}/{}.json", host_name, bw.name));

    if !run_config.force_rerun && out_path.exists() {
        info!("Skipping {} (already exists)", bw.name);
        return Ok(());
    }

    let block_number = bw.block_and_witness.block.number;
    let block_used_gas = bw.block_and_witness.block.gas_used;
    let mut stdin = Input::new();
    stdin.write(bw.block_and_witness.clone());
    stdin.write(bw.network);

    info!("Running {}", bw.name);
    let (execution, proving) = match run_config.action {
        Action::Execute => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm_ref.execute(&stdin)));
            let execution = match run {
                Ok(Ok(report)) => ExecutionMetrics::Success {
                    total_num_cycles: report.total_num_cycles,
                    region_cycles: report.region_cycles.into_iter().collect(),
                    execution_duration: report.execution_duration,
                },
                Ok(Err(e)) => ExecutionMetrics::Crashed(CrashInfo {
                    reason: e.to_string(),
                }),
                Err(panic_info) => ExecutionMetrics::Crashed(CrashInfo {
                    reason: get_panic_msg(panic_info),
                }),
            };
            (Some(execution), None)
        }
        Action::Prove => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm_ref.prove(&stdin)));
            let proving = match run {
                Ok(Ok((proof, report))) => ProvingMetrics::Success {
                    proof_size: proof.len(),
                    proving_time_ms: report.proving_time.as_millis(),
                },
                Ok(Err(e)) => ProvingMetrics::Crashed(CrashInfo {
                    reason: e.to_string(),
                }),
                Err(panic_info) => ProvingMetrics::Crashed(CrashInfo {
                    reason: get_panic_msg(panic_info),
                }),
            };
            (None, Some(proving))
        }
    };
    let report = BenchmarkRun {
        name: format!("{}-{}", bw.name, block_number),
        end_timestamp: chrono::Utc::now(),
        block_used_gas,
        execution,
        proving,
    };

    info!("Saving report for {}", bw.name);
    BenchmarkRun::to_path(out_path, &[report])?;

    Ok(())
}

fn get_panic_msg(panic_info: Box<dyn Any + Send>) -> String {
    panic_info
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| panic_info.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic occurred".to_string())
}
