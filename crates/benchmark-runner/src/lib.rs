use anyhow::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use witness_generator::{generate_stateless_witness, BlocksAndWitnesses};
use zkevm_metrics::WorkloadMetrics;
use zkvm_interface::{zkVM, Input};

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

/// Action specifies whether we should prove or execute
#[derive(Clone, Copy)]
pub enum Action {
    Prove,
    Execute,
}

pub fn run_benchmark_ere<V>(host_name: &str, zkvm_instance: V, action: Action) -> Result<()>
where
    V: zkVM + Sync,
{
    println!("Benchmarking `{}`…", host_name);
    let zkvm_ref = Arc::new(&zkvm_instance);
    let corpuses = generate_stateless_witness::generate();

    match action {
        Action::Execute => {
            // Use parallel iteration for execution
            corpuses.into_par_iter().try_for_each(|bw| -> Result<()> {
                process_corpus(bw, zkvm_ref.clone(), &action, host_name)
            })?;
        }
        Action::Prove => {
            // Use sequential iteration for proving
            corpuses.into_iter().try_for_each(|bw| -> Result<()> {
                process_corpus(bw, Arc::new(&*zkvm_ref), &action, host_name)
            })?;
        }
    }

    Ok(())
}

fn process_corpus<V>(
    bw: BlocksAndWitnesses,
    zkvm_ref: Arc<&V>,
    action: &Action,
    host_name: &str,
) -> Result<()>
where
    V: zkVM + Sync,
{
    println!(" {} ({} blocks)", bw.name, bw.blocks_and_witnesses.len());
    let mut reports = Vec::new();

    for ci in bw.blocks_and_witnesses {
        let block_number = ci.block.number;
        let mut stdin = Input::new();
        stdin.write(ci);
        stdin.write(bw.network);

        let workload_metrics = match action {
            Action::Execute => {
                let report = zkvm_ref.execute(&stdin)?;
                let region_cycles: HashMap<_, _> = report.region_cycles.into_iter().collect();
                WorkloadMetrics::Execution {
                    name: format!("{}-{}", bw.name, block_number),
                    total_num_cycles: report.total_num_cycles,
                    region_cycles,
                }
            }
            Action::Prove => {
                let (_, report) = zkvm_ref.prove(&stdin)?;
                WorkloadMetrics::Proving {
                    name: format!("{}-{}", bw.name, block_number),
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
