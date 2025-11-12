//! Runner for benchmark tests

use anyhow::{anyhow, bail, Context, Result};
use ere_dockerized::{zkVMKind, DockerizedCompiler, DockerizedzkVM};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{any::Any, panic};
use tracing::{error, info};

#[cfg(feature = "memory-tracking")]
use std::{sync::{Arc, Mutex}, thread, time::Duration as StdDuration};
#[cfg(feature = "memory-tracking")]
use zkevm_metrics::MemoryTracker;

use ere_zkvm_interface::{zkVM, Compiler, ProofKind, ProverResourceType, PublicValues};
use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};

use crate::guest_programs::{GuestFixture, OutputVerifierResult};

/// Holds the configuration for running benchmarks
#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Output folder where benchmark results will be stored
    pub output_folder: PathBuf,
    /// Optional subfolder within the output folder
    pub sub_folder: Option<String>,
    /// Action to perform: either proving or executing
    pub action: Action,
    /// Force rerun benchmarks even if output files already exist
    pub force_rerun: bool,
    /// Optional folder to dump input files
    pub dump_inputs_folder: Option<PathBuf>,
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
pub fn run_benchmark(
    ere_zkvm: &DockerizedzkVM,
    config: &RunConfig,
    inputs: impl IntoParallelIterator<Item: GuestFixture> + IntoIterator<Item: GuestFixture>,
) -> Result<()> {
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;
    match config.action {
        Action::Execute => inputs
            .into_par_iter()
            .try_for_each(|input| process_input(ere_zkvm, input, config))?,

        Action::Prove => inputs
            .into_iter()
            .try_for_each(|input| process_input(ere_zkvm, input, config))?,
    }

    Ok(())
}

/// Processes a single input through the zkVM
fn process_input(zkvm: &DockerizedzkVM, io: impl GuestFixture, config: &RunConfig) -> Result<()> {
    let zkvm_name = format!("{}-v{}", zkvm.name(), zkvm.sdk_version());
    let out_path = config
        .output_folder
        .join(config.sub_folder.as_deref().unwrap_or(""))
        .join(format!("{zkvm_name}/{}.json", io.name()));

    if !config.force_rerun && out_path.exists() {
        info!("Skipping {} (already exists)", &io.name());
        return Ok(());
    }

    let input = io.input()?;

    // Dump input if requested
    if let Some(ref dump_folder) = config.dump_inputs_folder {
        dump_input(
            input.stdin(),
            &io.name(),
            dump_folder,
            config.sub_folder.as_deref(),
        )?;
    }

    info!("Running {}", io.name());
    let (execution, proving) = match config.action {
        Action::Execute => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.execute(&input)));
            let execution = match run {
                Ok(Ok((public_values, report))) => {
                    verify_public_output(&io, &public_values)
                        .context("Failed to verify public output from execution")?;

                    ExecutionMetrics::Success {
                        total_num_cycles: report.total_num_cycles,
                        region_cycles: report.region_cycles.into_iter().collect(),
                        execution_duration: report.execution_duration,
                    }
                }
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
            #[cfg(feature = "memory-tracking")]
            let (memory_tracker, sample_handle) = {
                // Set up memory tracking
                let memory_tracker = Arc::new(Mutex::new(MemoryTracker::new()));
                let memory_tracker_clone = memory_tracker.clone();

                // Start memory tracking
                {
                    let mut tracker = memory_tracker.lock().unwrap();
                    tracker.start_tracking();
                }

                // Start background memory sampling thread
                let sample_handle = {
                    let tracker = memory_tracker_clone.clone();
                    thread::spawn(move || {
                        let start = std::time::Instant::now();
                        while start.elapsed().as_secs() < 3600 { // Max 1 hour timeout
                            {
                                let mut tracker = tracker.lock().unwrap();
                                tracker.sample_memory();
                            }
                            thread::sleep(StdDuration::from_millis(100)); // Sample every 100ms
                        }
                    })
                };

                (memory_tracker, sample_handle)
            };

            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                zkvm.prove(&input, ProofKind::Compressed)
            }));

            #[cfg(feature = "memory-tracking")]
            let final_tracker = {
                // Stop memory tracking
                drop(sample_handle);
                memory_tracker.lock().unwrap().clone()
            };

            let proving = match run {
                Ok(Ok((public_values, proof, report))) => {
                    verify_public_output(&io, &public_values)
                        .context("Failed to verify public output from proof")?;
                    let verif_public_values =
                        zkvm.verify(&proof).context("Failed to verify proof")?;
                    verify_public_output(&io, &verif_public_values)
                        .context("Failed to verify public output from proof verification")?;

                    ProvingMetrics::Success {
                        proof_size: proof.as_bytes().len(),
                        proving_time_ms: report.proving_time.as_millis(),
                        #[cfg(feature = "memory-tracking")]
                        peak_memory_usage_bytes: Some(final_tracker.get_peak_memory()),
                        #[cfg(not(feature = "memory-tracking"))]
                        peak_memory_usage_bytes: None,
                        #[cfg(feature = "memory-tracking")]
                        average_memory_usage_bytes: Some(final_tracker.get_average_memory()),
                        #[cfg(not(feature = "memory-tracking"))]
                        average_memory_usage_bytes: None,
                        #[cfg(feature = "memory-tracking")]
                        initial_memory_usage_bytes: Some(final_tracker.get_initial_memory()),
                        #[cfg(not(feature = "memory-tracking"))]
                        initial_memory_usage_bytes: None,
                    }
                }
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
        name: io.name(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: io.metadata(),
        execution,
        proving,
    };

    info!("Saving report {}", io.name());
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

/// Creates the requested zkVMs configured for the guest program and resources.
pub fn get_zkvm_instances(
    zkvms: &[zkVMKind],
    workspace_dir: &Path,
    guest_relative: &Path,
    resource: ProverResourceType,
    apply_patches: bool,
) -> Result<Vec<DockerizedzkVM>> {
    let mut instances = Vec::new();
    for zkvm in zkvms {
        if apply_patches {
            run_cargo_patch_command(zkvm.as_str(), workspace_dir)?;
        }
        let program = DockerizedCompiler::new(
            *zkvm,
            ere_dockerized::CompilerKind::RustCustomized,
            workspace_dir,
        )?
        .compile(&workspace_dir.join(guest_relative).join(zkvm.as_str()))?;
        instances.push(DockerizedzkVM::new(*zkvm, program, resource.clone())?);
    }
    Ok(instances)
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(zkvm_name: &str, workspace_path: &Path) -> Result<()> {
    info!("Running cargo {}...", zkvm_name);

    let output = Command::new("cargo")
        .arg(zkvm_name)
        .arg("--manifest-folder")
        .arg(workspace_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        error!(
            "cargo {} failed with exit code: {:?}",
            zkvm_name,
            output.status.code()
        );
        error!("stdout: {}", stdout);
        error!("stderr: {}", stderr);

        bail!("cargo {zkvm_name} command failed");
    }

    info!("cargo {zkvm_name} completed successfully");
    Ok(())
}

/// Dumps the raw input bytes to disk
fn dump_input(
    input: &[u8],
    name: &str,
    dump_folder: &Path,
    sub_folder: Option<&str>,
) -> Result<()> {
    let input_dir = dump_folder.join(sub_folder.unwrap_or(""));

    fs::create_dir_all(&input_dir).context(format!(
        "Failed to create directory: {}",
        input_dir.display()
    ))?;

    let input_path = input_dir.join(format!("{}.bin", name));

    // Only write if it doesn't exist (avoid duplicate writes across zkVMs)
    if !input_path.exists() {
        fs::write(&input_path, input)
            .context(format!("Failed to write input to {}", input_path.display()))?;
        info!("Dumped input to {}", input_path.display());
    }

    Ok(())
}

fn verify_public_output(io: &impl GuestFixture, public_values: &[u8]) -> Result<()> {
    match io.verify_public_values(public_values)? {
        OutputVerifierResult::Match => Ok(()),
        OutputVerifierResult::Mismatch(msg) => {
            Err(anyhow!("Output mismatch for {}: {msg}", io.name()))
        }
    }
}
