use anyhow::*;
use rayon::prelude::*;
use std::{fs, panic, sync::Arc};
use witness_generator::{witness_generator::WitnessGenerator, BlocksAndWitnesses};
use zkevm_metrics::WorkloadMetrics;
use zkvm_interface::{zkVM, Input};

#[deprecated(note = "this function is being phased out, use run_benchmark_ere")]
pub async fn run_benchmark<F>(
    elf_path: &'static [u8],
    metrics_path_prefix: &str,
    zkvm_executor: F,
    wg: &Box<dyn WitnessGenerator>,
) -> Result<()>
where
    F: Fn(&BlocksAndWitnesses, &'static [u8]) -> Vec<WorkloadMetrics> + Send + Sync,
{
    let generated_corpuses = wg.generate().await?;

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
    Ok(())
}

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
) where
    V: zkVM + Sync,
{
    println!("Benchmarking `{}`…", host_name);
    let zkvm_ref = Arc::new(&zkvm_instance);

    match action {
        Action::Execute => {
            // Use parallel iteration for execution
            corpuses.into_par_iter().for_each(|bw| {
                process_corpus_with_crash_handling(bw, zkvm_ref.clone(), &action, host_name);
            });
        }
        Action::Prove => {
            // Use sequential iteration for proving
            corpuses.into_iter().for_each(|bw| {
                process_corpus_with_crash_handling(bw, Arc::new(&*zkvm_ref), &action, host_name);
            });
        }
    }
}

fn process_corpus_with_crash_handling<V>(
    bw: &BlocksAndWitnesses,
    zkvm_ref: Arc<&V>,
    action: &Action,
    host_name: &str,
) where
    V: zkVM + Sync,
{
    let bench_name = bw.name.clone();
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        process_corpus(bw, zkvm_ref, action, host_name)
    }));

    let action_str = match action {
        Action::Execute => "execute",
        Action::Prove => "prove",
    };

    use std::result::Result::Ok;
    let crash_reason = match result {
        Ok(Ok(())) => {
            // Success, nothing to do
            return;
        }
        Ok(Err(e)) => {
            // Regular error - treat as crash
            format!("Error: {}", e)
        }
        Err(panic_info) => {
            // Panic - treat as crash
            let panic_msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic occurred".to_string()
            };
            format!("Panic: {}", panic_msg)
        }
    };

    eprintln!(
        "Benchmark CRASHED for {}: {}",
        bench_name.clone(),
        crash_reason
    );

    // Create crash metrics
    let crash_metrics = WorkloadMetrics::Crashed {
        name: bench_name.clone(),
        action: action_str.to_string(),
        reason: crash_reason.clone(),
    };

    // Save crash info to crash directory
    let crash_dir = format!(
        "{}/zkevm-metrics/{}/crash",
        env!("CARGO_WORKSPACE_DIR"),
        host_name
    );

    if let Err(e) = fs::create_dir_all(&crash_dir) {
        panic!("Failed to create crash directory: {}", e);
    }

    // Save crash metrics as JSON
    let crash_json_file = format!("{}/{}.json", crash_dir, &bench_name);
    if let Err(e) = WorkloadMetrics::to_path(&crash_json_file, &[crash_metrics]) {
        panic!("Failed to save crash metrics JSON: {}", e);
    } else {
        println!(
            "Recorded crash for corpus: {} in {}",
            bench_name, crash_json_file
        );
    }
}

fn process_corpus<V>(
    bw: &BlocksAndWitnesses,
    zkvm_ref: Arc<&V>,
    action: &Action,
    host_name: &str,
) -> Result<()>
where
    V: zkVM + Sync,
{
    // Take the last element, because benchmarks are setup in such a way that
    // We only want to benchmark the last block.
    let last_block_with_witness = match bw.blocks_and_witnesses.last() {
        Some(last_block) => last_block.clone(),
        None => panic!("unexpected test with no blocks {}", &bw.name),
    };

    let blocks_and_witnesses = vec![last_block_with_witness];

    println!(" {} ({} blocks)", bw.name, blocks_and_witnesses.len());
    let mut reports = Vec::new();

    for ci in blocks_and_witnesses {
        let block_number = ci.block.number;
        let mut stdin = Input::new();
        stdin.write(ci);
        stdin.write(bw.network);

        let workload_metrics = match action {
            Action::Execute => {
                let report = zkvm_ref.execute(&stdin)?;
                WorkloadMetrics::Execution {
                    name: format!("{}-{}", bw.name, block_number),
                    total_num_cycles: report.total_num_cycles,
                    region_cycles: report.region_cycles.into_iter().collect(),
                    execution_duration: report.execution_duration,
                }
            }
            Action::Prove => {
                let (proof, report) = zkvm_ref.prove(&stdin)?;
                WorkloadMetrics::Proving {
                    name: format!("{}-{}", bw.name, block_number),
                    proof_size: proof.len(),
                    proving_time_ms: report.proving_time.as_millis(),
                }
            }
        };
        reports.push(workload_metrics);
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
}
