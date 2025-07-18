//! Benchmark runner library for zkVM benchmarking

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod guest_programs;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::PathBuf;
use std::{any::Any, panic};
use tracing::info;

use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};
use zkvm_interface::zkVM;

use crate::guest_programs::{GuestInput, GuestInputMetadata};

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

/// Executes benchmarks for a given guest program type and zkVM
pub fn run_benchmark<M>(
    zkvm: Box<dyn zkVM + Sync>,
    config: &RunConfig,
    inputs: Vec<GuestInput<M>>,
) -> anyhow::Result<()>
where
    M: GuestInputMetadata,
{
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;
    match config.action {
        Action::Execute => inputs
            .par_iter()
            .try_for_each(|input| process_input(&zkvm, input, config))?,

        Action::Prove => inputs
            .iter()
            .try_for_each(|input| process_input(&zkvm, input, config))?,
    }

    Ok(())
}

/// Processes a single input through the zkVM
fn process_input<V, M>(zkvm: &V, input: &GuestInput<M>, config: &RunConfig) -> anyhow::Result<()>
where
    V: zkVM + Sync,
    M: GuestInputMetadata,
{
    let zkvm_name = format!("{}-v{}", zkvm.name(), zkvm.sdk_version());
    let out_path = config
        .output_folder
        .join(format!("{zkvm_name}/{}.json", input.name));

    if !config.force_rerun && out_path.exists() {
        info!("Skipping {} (already exists)", &input.name);
        return Ok(());
    }

    info!("Running {}", input.name);
    let (execution, proving) = match config.action {
        Action::Execute => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.execute(&input.stdin)));
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
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.prove(&input.stdin)));
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
        name: input.name.clone(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: input.metadata.clone(),
        execution,
        proving,
    };

    info!("Saving report {}", input.name);
    report.to_path(out_path)?;

    Ok(())
}

fn get_panic_msg(panic_info: Box<dyn Any + Send>) -> String {
    panic_info
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| panic_info.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic occurred".to_string())
}
