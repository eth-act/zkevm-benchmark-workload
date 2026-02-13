//! Runner for benchmark tests

use anyhow::{anyhow, Context, Result};
use ere_dockerized::{zkVMKind, DockerizedzkVM, SerializedProgram};
use ere_guests_downloader::Downloader;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs;
use std::path::{Path, PathBuf};
use std::{any::Any, panic};
use tracing::info;

use ere_zkvm_interface::{zkVM, ProofKind, ProverResourceType};
use zkevm_metrics::{BenchmarkRun, CrashInfo, ExecutionMetrics, HardwareInfo, ProvingMetrics};

use crate::guest_programs::{GuestFixture, OutputVerifierResult};
use crate::zisk_profiling::run_profiling;

pub use crate::zisk_profiling::ProfileConfig;

/// Default version tag for guest programs
const DEFAULT_GUEST_VERSION: &str = "v0.5.0";

/// A zkVM instance bundled with ELF bytes (used for profiling).
pub struct ZkVMInstance {
    /// The dockerized zkVM instance
    pub zkvm: DockerizedzkVM,
    /// Raw ELF bytes of the guest program
    pub elf: Vec<u8>,
}

impl std::fmt::Debug for ZkVMInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZkVMInstance")
            .field("zkvm", &self.zkvm.name())
            .field("elf_len", &self.elf.len())
            .finish()
    }
}

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
    /// Optional Zisk profiling configuration
    pub zisk_profile_config: Option<ProfileConfig>,
    /// Optional folder to save proof artifacts for later verification
    pub save_proofs_folder: Option<PathBuf>,
}

/// Action specifies whether we should prove or execute
#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Generate a proof for the zkVM execution
    Prove,
    /// Only execute the zkVM without proving
    Execute,
    /// Verify proofs loaded from disk
    Verify,
}

/// Executes benchmarks for a given guest program type and zkVM
pub fn run_benchmark(
    instance: &ZkVMInstance,
    config: &RunConfig,
    inputs: impl IntoParallelIterator<Item: GuestFixture> + IntoIterator<Item: GuestFixture>,
) -> Result<()> {
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;

    let zkvm = &instance.zkvm;
    let elf = &instance.elf;
    match config.action {
        Action::Execute => inputs
            .into_par_iter()
            .try_for_each(|input| process_input(zkvm, input, config, elf))?,

        Action::Prove => inputs
            .into_iter()
            .try_for_each(|input| process_input(zkvm, input, config, elf))?,

        Action::Verify => {
            return Err(anyhow!(
                "run_benchmark should not be called with Action::Verify, use run_verify_from_disk"
            ))
        }
    }

    Ok(())
}

/// Processes a single input through the zkVM
fn process_input(
    zkvm: &DockerizedzkVM,
    io: impl GuestFixture,
    config: &RunConfig,
    elf: &[u8],
) -> Result<()> {
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
            // Run Zisk profiling if configured
            if let Some(profile_config) = &config.zisk_profile_config {
                run_profiling(
                    profile_config,
                    elf,
                    input.stdin(),
                    &io.name(),
                    config.sub_folder.as_deref(),
                )?;
            }

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
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                zkvm.prove(&input, ProofKind::Compressed)
            }));
            let proving = match run {
                Ok(Ok((public_values, proof, report))) => {
                    verify_public_output(&io, &public_values)
                        .context("Failed to verify public output from proof")?;

                    // Save proof to disk if requested
                    if let Some(ref proofs_folder) = config.save_proofs_folder {
                        save_proof(
                            &proof,
                            &io.name(),
                            &zkvm_name,
                            proofs_folder,
                            config.sub_folder.as_deref(),
                        )?;
                    }

                    let verify_start = std::time::Instant::now();
                    let verif_public_values =
                        zkvm.verify(&proof).context("Failed to verify proof")?;
                    let verification_time_ms = verify_start.elapsed().as_millis();
                    verify_public_output(&io, &verif_public_values)
                        .context("Failed to verify public output from proof verification")?;

                    ProvingMetrics::Success {
                        proof_size: proof.as_bytes().len(),
                        proving_time_ms: report.proving_time.as_millis(),
                        verification_time_ms,
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
        Action::Verify => {
            return Err(anyhow!(
                "process_input should not be called with Action::Verify, use run_verify_from_disk"
            ))
        }
    };

    let report = BenchmarkRun {
        name: io.name(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: io.metadata(),
        execution,
        proving,
        verification: None,
    };

    info!("Saving report {}", io.name());
    report.to_path(out_path)?;

    Ok(())
}

pub(crate) fn get_panic_msg(panic_info: Box<dyn Any + Send>) -> String {
    panic_info
        .downcast_ref::<&str>()
        .map(|s| s.to_string())
        .or_else(|| panic_info.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "Unknown panic occurred".to_string())
}

/// Creates the requested EL/zkVMs ere instances.
pub async fn get_el_zkvm_instances(
    el: &str,
    zkvms: &[zkVMKind],
    resource: ProverResourceType,
    bin_path: Option<&Path>,
) -> Result<Vec<ZkVMInstance>> {
    let guest_name_prefix = format!("stateless-validator-{el}");
    get_guest_zkvm_instances(&guest_name_prefix, zkvms, resource, bin_path).await
}

/// Creates the requested guest program zkVMs ere instances.
pub async fn get_guest_zkvm_instances(
    guest_name_prefix: &str,
    zkvms: &[zkVMKind],
    resource: ProverResourceType,
    bin_path: Option<&Path>,
) -> Result<Vec<ZkVMInstance>> {
    let mut instances = Vec::new();
    for zkvm in zkvms {
        let guest_name = format!("{}-{}", guest_name_prefix, zkvm.as_str());
        let (program, elf) = load_program(&guest_name, bin_path).await?;
        let zkvm = DockerizedzkVM::new(*zkvm, program, resource.clone())
            .with_context(|| format!("Failed to initialize DockerizedzkVM, kind {zkvm}"))?;
        instances.push(ZkVMInstance { zkvm, elf });
    }
    Ok(instances)
}

async fn load_program(
    guest_name: &str,
    bin_path: Option<&Path>,
) -> Result<(SerializedProgram, Vec<u8>)> {
    if let Some(path) = bin_path {
        let bytes = fs::read(path.join(guest_name))
            .with_context(|| format!("Failed to read program from path: {}", path.display()))?;
        let elf = fs::read(path.join(format!("{guest_name}.elf")))
            .with_context(|| format!("Failed to read ELF from path: {}", path.display()))?;
        return Ok((SerializedProgram(bytes), elf));
    }

    let downloader = Downloader::from_tag(DEFAULT_GUEST_VERSION)
        .await
        .context("Failed to create guest program downloader")?;
    let compiled = downloader
        .download(guest_name)
        .await
        .with_context(|| format!("Failed to download guest program: {guest_name}"))?;

    Ok((SerializedProgram(compiled.program), compiled.elf))
}

/// Dumps the raw input bytes to disk
fn dump_input(
    input: &[u8],
    name: &str,
    dump_folder: &Path,
    sub_folder: Option<&str>,
) -> Result<()> {
    let input_dir = dump_folder.join(sub_folder.unwrap_or(""));

    fs::create_dir_all(&input_dir)
        .with_context(|| format!("Failed to create directory: {}", input_dir.display()))?;

    let input_path = input_dir.join(format!("{name}.bin"));

    // Only write if it doesn't exist (avoid duplicate writes across zkVMs)
    if !input_path.exists() {
        fs::write(&input_path, input)
            .with_context(|| format!("Failed to write input to {}", input_path.display()))?;
        info!("Dumped input to {}", input_path.display());
    }

    Ok(())
}

/// Saves a proof's raw bytes to disk
fn save_proof(
    proof: &ere_zkvm_interface::Proof,
    name: &str,
    zkvm_name: &str,
    proofs_folder: &Path,
    sub_folder: Option<&str>,
) -> Result<()> {
    let proof_dir = proofs_folder.join(sub_folder.unwrap_or("")).join(zkvm_name);

    fs::create_dir_all(&proof_dir)
        .with_context(|| format!("Failed to create directory: {}", proof_dir.display()))?;

    let proof_path = proof_dir.join(format!("{name}.proof"));
    fs::write(&proof_path, proof.as_bytes())
        .with_context(|| format!("Failed to write proof to {}", proof_path.display()))?;
    info!("Saved proof to {}", proof_path.display());

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
