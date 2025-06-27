use rayon::prelude::*;
use std::{any::Any, panic, sync::Arc};
use witness_generator::BlocksAndWitnesses;
use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};
use zkvm_interface::{zkVM, Input};

/// Action specifies whether we should prove or execute
#[derive(Clone, Copy)]
pub enum Action {
    Prove,
    Execute,
}

pub fn run_benchmark_ere<V>(
    host_name: &str,
    zkvm_instance: V,
    action: Action,
    corpuses: &[BlocksAndWitnesses],
) -> anyhow::Result<()>
where
    V: zkVM + Sync,
{
    HardwareInfo::detect().to_path(format!(
        "{}/zkevm-metrics/hardware.json",
        env!("CARGO_WORKSPACE_DIR")
    ))?;

    println!("Benchmarking `{}`…", host_name);
    let zkvm_ref = Arc::new(&zkvm_instance);

    match action {
        Action::Execute => {
            // Use parallel iteration for execution
            corpuses
                .into_par_iter()
                .try_for_each(|bw| process_corpus(bw, zkvm_ref.clone(), action, host_name))?;
        }
        Action::Prove => {
            // Use sequential iteration for proving
            corpuses
                .into_iter()
                .try_for_each(|bw| process_corpus(bw, zkvm_ref.clone(), action, host_name))?;
        }
    }
    Ok(())
}

fn process_corpus<V>(
    bw: &BlocksAndWitnesses,
    zkvm_ref: Arc<V>,
    action: Action,
    host_name: &str,
) -> anyhow::Result<()>
where
    V: zkVM + Sync,
{
    // Take the last element, because benchmarks are setup in such a way that
    // We only want to benchmark the last block.
    let last_block_with_witness = bw
        .blocks_and_witnesses
        .last()
        .ok_or_else(|| anyhow::anyhow!("Unexpected test with no blocks: {}", &bw.name))?
        .clone();

    let blocks_and_witnesses = vec![last_block_with_witness];

    println!(" {} ({} blocks)", bw.name, blocks_and_witnesses.len());
    let mut reports = Vec::new();

    for ci in blocks_and_witnesses {
        let block_number = ci.block.number;
        let block_used_gas = ci.block.gas_used;
        let mut stdin = Input::new();
        stdin.write(ci);
        stdin.write(bw.genesis.clone());

        let (execution, proving) = match action {
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
        reports.push(BenchmarkRun {
            name: format!("{}-{}", bw.name, block_number),
            block_used_gas,
            execution,
            proving,
        });
    }

    let out_path = format!(
        "{}/zkevm-metrics/{}/{}.json",
        env!("CARGO_WORKSPACE_DIR"),
        host_name,
        bw.name
    );
    BenchmarkRun::to_path(out_path, &reports)?;
    println!("wrote {} reports", reports.len());
    Ok(())
}

fn get_panic_msg(panic_info: Box<dyn Any + Send>) -> String {
    panic_info
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| panic_info.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic occurred".to_string())
}
