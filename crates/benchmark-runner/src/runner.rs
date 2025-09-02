//! Runner for benchmark tests

use anyhow::{anyhow, Context, Result};
use ere_dockerized::{EreDockerizedCompiler, EreDockerizedzkVM, ErezkVM};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{any::Any, panic};
use tracing::{error, info};

use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};
use zkvm_interface::{zkVM, Compiler, ProverResourceType, PublicValues};

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier};

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
pub fn run_benchmark<M, OV>(
    ere_zkvm: &EreDockerizedzkVM,
    config: &RunConfig,
    inputs: Vec<GuestIO<M, OV>>,
) -> Result<()>
where
    M: GuestMetadata,
    OV: OutputVerifier,
{
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;
    match config.action {
        Action::Execute => inputs
            .par_iter()
            .try_for_each(|input| process_input(ere_zkvm, input, config))?,

        Action::Prove => inputs
            .iter()
            .try_for_each(|input| process_input(ere_zkvm, input, config))?,
    }

    Ok(())
}

/// Processes a single input through the zkVM
fn process_input<M, OV>(
    zkvm: &EreDockerizedzkVM,
    io: &GuestIO<M, OV>,
    config: &RunConfig,
) -> Result<()>
where
    M: GuestMetadata,
    OV: OutputVerifier,
{
    let zkvm_name = format!("{}-v{}", zkvm.name(), zkvm.sdk_version());
    let out_path = config
        .output_folder
        .join(format!("{zkvm_name}/{}.json", io.name));

    if !config.force_rerun && out_path.exists() {
        info!("Skipping {} (already exists)", &io.name);
        return Ok(());
    }

    info!("Running {}", io.name);
    let (execution, proving) = match config.action {
        Action::Execute => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.execute(&io.input)));
            let execution = match run {
                Ok(Ok((public_values, report))) => {
                    verify_public_output(&io.name, zkvm.zkvm(), &public_values, &io.output)?;

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
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.prove(&io.input)));
            let proving = match run {
                Ok(Ok((public_values, proof, report))) => {
                    verify_public_output(&io.name, zkvm.zkvm(), &public_values, &io.output)?;
                    let verif_public_values =
                        zkvm.verify(&proof).context("Failed to verify proof")?;
                    verify_public_output(&io.name, zkvm.zkvm(), &verif_public_values, &io.output)?;

                    ProvingMetrics::Success {
                        proof_size: proof.len(),
                        proving_time_ms: report.proving_time.as_millis(),
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
        name: io.name.clone(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: io.metadata.clone(),
        execution,
        proving,
    };

    info!("Saving report {}", io.name);
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
    zkvms: &[ErezkVM],
    workspace_dir: &Path,
    guest_relative: &Path,
    resource: ProverResourceType,
    apply_patches: bool,
) -> Result<Vec<EreDockerizedzkVM>, Box<dyn std::error::Error>> {
    let mut instances = Vec::new();
    for zkvm in zkvms {
        if apply_patches {
            run_cargo_patch_command(zkvm.as_str(), workspace_dir)?;
        }
        let program = EreDockerizedCompiler::new(*zkvm, workspace_dir)?
            .compile(&workspace_dir.join(guest_relative).join(zkvm.as_str()))?;
        instances.push(EreDockerizedzkVM::new(*zkvm, program, resource.clone())?);
    }
    Ok(instances)
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(
    zkvm_name: &str,
    workspace_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
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

        return Err(format!("cargo {zkvm_name} command failed").into());
    }

    info!("cargo {zkvm_name} completed successfully");
    Ok(())
}

fn verify_public_output(
    name: &str,
    zkvm: ErezkVM,
    public_values: &PublicValues,
    output_verifier: &impl OutputVerifier,
) -> Result<()> {
    if !output_verifier.check_serialized(zkvm, public_values)? {
        return Err(anyhow!("Output mismatch for {}", name));
    }
    Ok(())
}
