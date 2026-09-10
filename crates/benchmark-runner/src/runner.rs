//! Runner for benchmark tests

use anyhow::{anyhow, bail, Context, Result};
use ere_cluster_client_zisk::{ZiskClusterClient, ZiskProof};
use ere_dockerized::{
    codec::{Decode, Encode},
    zkVMKind, zkVMVerifier, CostEstimation, DockerizedzkVM, DockerizedzkVMConfig, Elf,
    EncodedProof, Input, ProverResource, PublicValues,
};
use ere_util_tokio::block_on;
use rayon::iter::{ParallelBridge, ParallelIterator};
use sha2::{Digest, Sha256};
use stateless_validator_catalog::StatelessValidatorKind;
use stateless_validator_downloader::{CompiledGuest, Downloader};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{any::Any, env, panic};
use std::{collections::BTreeMap, fs};
use tokio::time::Instant;
use tracing::{info, warn};

use zkevm_metrics::{
    BenchmarkRun, CostEstimationContext, CostEstimationMetrics, CrashInfo, ExecutionMetrics,
    HardwareInfo, ProvingMetrics,
};

use crate::zisk_profiling::{run_profiling, ProfileOutcome};
use crate::{guest_programs::GuestFixture, stateless_validator::ExecutionClient};

pub use crate::zisk_profiling::ProfileConfig;

/// How to resolve downloaded guest binaries, derived from the resolved
/// ere-guests dependency in Cargo.lock at build time.
const ERE_GUESTS_DOWNLOAD_KIND: &str = env!("ERE_GUESTS_DOWNLOAD_KIND");
/// Tag or commit SHA matching [`ERE_GUESTS_DOWNLOAD_KIND`].
const ERE_GUESTS_DOWNLOAD_VALUE: &str = env!("ERE_GUESTS_DOWNLOAD_VALUE");

/// Source used to resolve compiled guest programs.
#[derive(Debug, Clone)]
pub enum GuestProgramSource {
    /// Resolve guest programs from the configured ere-guests dependency.
    Default,
    /// Resolve guest programs from a local directory.
    LocalPath(PathBuf),
    /// Resolve guest programs from a remote base URL.
    ArtifactBaseUrl(String),
}

impl GuestProgramSource {
    /// Returns a stable label for externally supplied guest artifacts.
    pub fn version_label(&self) -> Option<String> {
        match self {
            Self::Default => None,
            Self::LocalPath(path) => path
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.is_empty())
                .map(std::string::ToString::to_string),
            Self::ArtifactBaseUrl(url) => artifact_base_url_label(url),
        }
    }
}

/// A zkVM instance bundled with ELF bytes (used for profiling).
pub enum ZkVMInstance {
    /// Dockerized zkVM instance
    Dockerized {
        /// zkVM instance
        zkvm: DockerizedzkVM,
        /// Guest identity and estimator configuration captured before startup.
        cost_context: Box<CostEstimationContext>,
        /// ELF of Zisk guest with feature `cycle-scope` enabled.
        /// `Some` only if the guest is a Zisk guest.
        profiling_elf: Option<Elf>,
    },
    /// Remote Zisk proving cluster client.
    ZiskClusterClient {
        /// gRPC client connected to the remote Zisk cluster.
        client: ZiskClusterClient,
        /// Per-request prove timeout, propagated from the `DockerizedzkVMConfig`
        /// prove timeout. Defaults to 3 minutes.
        prove_timeout: Duration,
        /// ELF of Zisk guest with feature `cycle-scope` enabled.
        /// `Some` only if the guest is a Zisk guest.
        profiling_elf: Option<Elf>,
    },
}

impl ZkVMInstance {
    /// Returns the zkVM kind.
    pub fn zkvm_kind(&self) -> zkVMKind {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.zkvm_kind(),
            Self::ZiskClusterClient { .. } => zkVMKind::Zisk,
        }
    }

    /// Returns the zkVM name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.name(),
            Self::ZiskClusterClient { client, .. } => client.verifier().name(),
        }
    }

    /// Returns the zkVM SDK version.
    pub fn sdk_version(&self) -> &'static str {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.sdk_version(),
            Self::ZiskClusterClient { client, .. } => client.verifier().sdk_version(),
        }
    }

    /// Returns the ELF for Zisk profiling.
    pub const fn profiling_elf(&self) -> Option<&Elf> {
        match self {
            Self::Dockerized { profiling_elf, .. }
            | Self::ZiskClusterClient { profiling_elf, .. } => profiling_elf.as_ref(),
        }
    }

    /// Executes the guest program without proving.
    pub fn execute(&self, input: &Input) -> Result<(PublicValues, Duration)> {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.execute(input),
            Self::ZiskClusterClient { .. } => {
                bail!("ZiskClusterClient does not support Action::Execute")
            }
        }
    }

    /// Executes the guest with the cost estimator.
    pub fn estimate_cost(&self, input: &Input) -> Result<(PublicValues, CostEstimation)> {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.execute_estimated_cost(input),
            Self::ZiskClusterClient { .. } => {
                bail!("ZiskClusterClient does not support Action::EstimateCost")
            }
        }
    }

    fn cost_context(&self, input: &Input) -> Result<CostEstimationContext> {
        match self {
            Self::Dockerized { cost_context, .. } => {
                let mut context = (**cost_context).clone();
                context.input_sha256 = hex::encode(Sha256::digest(input.stdin()));
                Ok(context)
            }
            Self::ZiskClusterClient { .. } => {
                bail!("ZiskClusterClient does not support cost estimation")
            }
        }
    }

    /// Generates a proof for the guest program with the given input.
    pub fn prove(&self, input: &Input) -> Result<(PublicValues, EncodedProof, Duration)> {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.prove(input),
            Self::ZiskClusterClient {
                client,
                prove_timeout,
                ..
            } => {
                let deadline = Instant::now() + *prove_timeout;
                let (proof, proving_time) = block_on(client.prove(input, deadline))?;
                let (_, public_values) = proof.program_vk_and_public_values()?;
                let proof = proof.encode_to_vec()?;
                Ok((public_values, EncodedProof(proof), proving_time))
            }
        }
    }

    /// Verifies a proof and returns the public values it commits to.
    pub fn verify(&self, proof: &EncodedProof) -> Result<PublicValues> {
        match self {
            Self::Dockerized { zkvm, .. } => zkvm.verify(proof),
            Self::ZiskClusterClient { client, .. } => {
                let proof = ZiskProof::decode_from_slice(&proof.0)?;
                Ok(client.verifier().verify(&proof)?)
            }
        }
    }
}

impl std::fmt::Debug for ZkVMInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dockerized { zkvm, .. } => f
                .debug_struct("Dockerized")
                .field("zkvm", &zkvm.name())
                .field("sdk_version", &zkvm.sdk_version())
                .field("program_vk", &hex::encode(&zkvm.program_vk().0))
                .finish(),
            Self::ZiskClusterClient { client, .. } => f
                .debug_struct("ZiskClusterClient")
                .field("zkvm", &client.verifier().name())
                .field("sdk_version", &client.verifier().sdk_version())
                .field(
                    "program_vk",
                    &hex::encode(client.program_vk().encode_to_vec().expect("infallible")),
                )
                .finish(),
        }
    }
}

/// Holds the configuration for running benchmarks
#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Output folder where benchmark results will be stored
    pub output_folder: PathBuf,
    /// Optional subfolder within the output folder
    pub sub_folder: Option<String>,
    /// Action whose result to add or replace.
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

/// Benchmark action to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Generate a proof for the zkVM execution
    Prove,
    /// Only execute the zkVM without proving
    Execute,
    /// Estimate proving costs without generating a proof.
    EstimateCost,
    /// Verify proofs loaded from disk
    Verify,
}

/// Executes benchmarks from a lazy iterator of fixtures.
pub fn run_benchmark_iter<I>(instance: &ZkVMInstance, config: &RunConfig, inputs: I) -> Result<()>
where
    I: Iterator<Item = Result<Box<dyn GuestFixture>>> + Send,
{
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;

    match config.action {
        Action::Execute | Action::EstimateCost => inputs.par_bridge().try_for_each(|input| {
            let input = input?;
            process_input(instance, input, config)
        })?,

        Action::Prove => inputs.into_iter().try_for_each(|input| {
            let input = input?;
            process_input(instance, input, config)
        })?,

        Action::Verify => {
            return Err(anyhow!(
                "run_benchmark_iter should not be called with Action::Verify, use run_verify_from_disk"
            ));
        }
    }

    Ok(())
}

fn benchmark_zkvm_name(zkvm: &ZkVMInstance) -> String {
    format!("{}-{}", zkvm.name(), zkvm.sdk_version())
}

fn benchmark_output_dir_for_name(config: &RunConfig, zkvm_name: &str) -> PathBuf {
    config
        .output_folder
        .join(config.sub_folder.as_deref().unwrap_or(""))
        .join(zkvm_name)
}

fn benchmark_output_path_for_name(
    config: &RunConfig,
    zkvm_name: &str,
    fixture_name: &str,
) -> PathBuf {
    benchmark_output_dir_for_name(config, zkvm_name).join(format!("{fixture_name}.json"))
}

/// Returns the output directory for a given zkVM benchmark run.
pub fn benchmark_output_dir(zkvm: &ZkVMInstance, config: &RunConfig) -> PathBuf {
    benchmark_output_dir_for_name(config, &benchmark_zkvm_name(zkvm))
}

/// Returns the output path for a given fixture within a zkVM benchmark run.
pub fn benchmark_output_path(
    zkvm: &ZkVMInstance,
    config: &RunConfig,
    fixture_name: &str,
) -> PathBuf {
    benchmark_output_path_for_name(config, &benchmark_zkvm_name(zkvm), fixture_name)
}

/// Processes a single input through the zkVM
fn process_input(zkvm: &ZkVMInstance, io: impl GuestFixture, config: &RunConfig) -> Result<()> {
    let zkvm_name = benchmark_zkvm_name(zkvm);
    let fixture_name = io.name();
    let out_path = benchmark_output_path_for_name(config, &zkvm_name, &fixture_name);

    if should_skip_action(&out_path, config.action, config.force_rerun)? {
        info!("Skipping {} (already exists)", fixture_name);
        return Ok(());
    }

    let input = io.input()?;

    // Dump input if requested
    if let Some(ref dump_folder) = config.dump_inputs_folder {
        dump_input(
            input.stdin(),
            &fixture_name,
            dump_folder,
            config.sub_folder.as_deref(),
        )?;
    }

    info!("Running {}", fixture_name);
    let (execution, proving, cost_estimation) = match config.action {
        Action::Execute => {
            // Run Zisk profiling if configured
            if let Some(profile_config) = &config.zisk_profile_config {
                let Some(profiling_elf) = zkvm.profiling_elf() else {
                    bail!("Zisk profiling configured but profiling ELF not found")
                };
                let outcome = run_profiling(
                    profile_config,
                    profiling_elf,
                    input.stdin(),
                    &fixture_name,
                    config.sub_folder.as_deref(),
                );
                if let ProfileOutcome::Failed(message) = outcome {
                    warn!(
                        "Zisk profiling failed for {} but benchmark execution will continue: {}",
                        fixture_name, message
                    );
                }
            }

            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.execute(&input)));
            let execution = match run {
                Ok(Ok((public_values, execution_duration))) => {
                    let output_matched = public_output_matched(&io, &public_values)
                        .context("Failed to compare public output from execution")?;

                    ExecutionMetrics::Success {
                        output_matched,
                        execution_duration,
                    }
                }
                Ok(Err(e)) => ExecutionMetrics::Crashed(CrashInfo {
                    reason: e.to_string(),
                }),
                Err(panic_info) => ExecutionMetrics::Crashed(CrashInfo {
                    reason: get_panic_msg(panic_info),
                }),
            };
            (Some(execution), None, None)
        }
        Action::Prove => {
            let run = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.prove(&input)));
            let proving = match run {
                Ok(Ok((public_values, proof, proving_time))) => {
                    let prover_output_matched = public_output_matched(&io, &public_values)
                        .context("Failed to compare public output from proof")?;

                    // Save proof to disk if requested
                    if let Some(ref proofs_folder) = config.save_proofs_folder {
                        save_proof(
                            &proof,
                            &fixture_name,
                            &zkvm_name,
                            proofs_folder,
                            config.sub_folder.as_deref(),
                        )?;
                    }

                    let verify_start = std::time::Instant::now();
                    let verif_public_values =
                        zkvm.verify(&proof).context("Failed to verify proof")?;
                    let verification_time_ms = verify_start.elapsed().as_millis();
                    let verifier_output_matched = public_output_matched(&io, &verif_public_values)
                        .context("Failed to compare public output from proof verification")?;

                    ProvingMetrics::Success {
                        output_matched: prover_output_matched && verifier_output_matched,
                        proof_size: proof.len(),
                        proving_time_ms: proving_time.as_millis(),
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
            (None, Some(proving), None)
        }
        Action::EstimateCost => {
            let context = zkvm.cost_context(&input)?;
            let metrics = collect_cost_estimation(&io, context, || zkvm.estimate_cost(&input))?;
            (None, None, Some(metrics))
        }
        Action::Verify => {
            return Err(anyhow!(
                "process_input should not be called with Action::Verify, use run_verify_from_disk"
            ));
        }
    };

    let report = BenchmarkRun {
        name: fixture_name.clone(),
        timestamp_completed: zkevm_metrics::chrono::Utc::now(),
        metadata: io.metadata(),
        execution,
        proving,
        verification: None,
        cost_estimation,
    };

    info!("Saving report {}", fixture_name);
    report.merge_to_path(&out_path).with_context(|| {
        format!(
            "Failed to merge metrics at {}; repair or move the existing file before rerunning",
            out_path.display()
        )
    })?;

    Ok(())
}

/// Checks whether the selected action already has a result.
pub(crate) fn should_skip_action(path: &Path, action: Action, force: bool) -> Result<bool> {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err).with_context(|| format!("Failed to read {}", path.display())),
    };
    let run: BenchmarkRun<serde_json::Value> =
        serde_json::from_str(&contents).with_context(|| {
            format!(
                "Invalid metrics at {}; repair or move this file before rerunning",
                path.display()
            )
        })?;
    Ok(!force
        && match action {
            Action::Execute => run.execution.is_some(),
            Action::Prove => run.proving.is_some(),
            Action::Verify => run.verification.is_some(),
            Action::EstimateCost => run.cost_estimation.is_some(),
        })
}

fn collect_cost_estimation(
    io: &impl GuestFixture,
    context: CostEstimationContext,
    estimate: impl FnOnce() -> Result<(PublicValues, CostEstimation)>,
) -> Result<CostEstimationMetrics> {
    Ok(
        match panic::catch_unwind(panic::AssertUnwindSafe(estimate)) {
            Ok(Ok((public_values, report))) => CostEstimationMetrics::Success {
                output_matched: public_output_matched(io, &public_values)
                    .context("Failed to compare public output from cost estimation")?,
                cost: report.cost,
                peak_heap_bytes: report.peak_heap_bytes,
                context: Box::new(context),
            },
            Ok(Err(err)) => CostEstimationMetrics::Crashed(CrashInfo {
                reason: err.to_string(),
            }),
            Err(err) => CostEstimationMetrics::Crashed(CrashInfo {
                reason: get_panic_msg(err),
            }),
        },
    )
}

fn estimator_settings(
    zkvm: zkVMKind,
    env_value: impl Fn(&str) -> Option<String>,
) -> BTreeMap<String, String> {
    let heap_start = if zkvm == zkVMKind::Zisk {
        "_heap_bottom"
    } else {
        "_end"
    };
    let mut settings = BTreeMap::from([(
        "heap_start".to_owned(),
        env_value("ERE_COST_ESTIMATION_HEAP_START").unwrap_or_else(|| heap_start.to_owned()),
    )]);
    match zkvm {
        zkVMKind::Zisk => {
            settings.insert(
                "heap_end".to_owned(),
                env_value("ERE_COST_ESTIMATION_HEAP_END").unwrap_or_else(|| "_heap_top".to_owned()),
            );
        }
        zkVMKind::OpenVM => {
            let memory = env_value("ERE_OPENVM_SEGMENT_MEMORY")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(29 << 29);
            settings.insert("segment_memory_bytes".to_owned(), memory.to_string());
        }
        zkVMKind::SP1 => {}
    }
    settings
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
    el: ExecutionClient,
    zkvms: &[zkVMKind],
    resource: ProverResource,
    zkvm_config: DockerizedzkVMConfig,
    guest_source: &GuestProgramSource,
) -> Result<Vec<ZkVMInstance>> {
    get_guest_zkvm_instances(el, zkvms, resource, zkvm_config, guest_source).await
}

/// Creates the requested guest program zkVMs ere instances.
pub async fn get_guest_zkvm_instances(
    el: ExecutionClient,
    zkvms: &[zkVMKind],
    resource: ProverResource,
    zkvm_config: DockerizedzkVMConfig,
    guest_source: &GuestProgramSource,
) -> Result<Vec<ZkVMInstance>> {
    for zkvm in zkvms {
        el.validate_zkvm(*zkvm)?;
    }
    let mut instances = Vec::new();
    for zkvm in zkvms {
        let compiled = load_compiled(el, *zkvm, guest_source).await?;
        let instance = match &resource {
            ProverResource::Cpu | ProverResource::Gpu => {
                let cost_context = CostEstimationContext {
                    execution_client: el.as_ref().to_lowercase(),
                    execution_client_version: el.version()?.to_owned(),
                    zkvm: zkvm.as_str().to_owned(),
                    sdk_version: zkvm.sdk_version().to_owned(),
                    ere_revision: ere_dockerized::DOCKER_IMAGE_TAG.to_owned(),
                    elf_sha256: hex::encode(Sha256::digest(&compiled.elf)),
                    input_sha256: String::new(),
                    estimator_settings: estimator_settings(*zkvm, |name| env::var(name).ok()),
                };
                let kind = *zkvm;
                let resource = resource.clone();
                let zkvm_config = zkvm_config.clone();
                // Ere performs blocking container startup and health checks here.
                // Keep that work off the async caller's runtime thread.
                let zkvm = tokio::task::spawn_blocking(move || {
                    DockerizedzkVM::new(kind, Elf(compiled.elf), resource, zkvm_config)
                })
                .await
                .context("DockerizedzkVM initialization task failed")?
                .with_context(|| format!("Failed to initialize DockerizedzkVM, kind {zkvm}"))?;
                ZkVMInstance::Dockerized {
                    zkvm,
                    cost_context: Box::new(cost_context),
                    profiling_elf: compiled.profiling_elf.map(Elf),
                }
            }
            ProverResource::Cluster(cfg) if *zkvm == zkVMKind::Zisk => {
                const DEFAULT_PROVE_TIMEOUT: Duration = Duration::from_mins(3);

                let client = ZiskClusterClient::new(cfg, Elf(compiled.elf.clone()))
                    .await
                    .map_err(|e| anyhow!("Failed to connect to Zisk cluster: {e}"))?;
                ZkVMInstance::ZiskClusterClient {
                    client,
                    prove_timeout: zkvm_config.prove_timeout.unwrap_or(DEFAULT_PROVE_TIMEOUT),
                    profiling_elf: compiled.profiling_elf.map(Elf),
                }
            }
            ProverResource::Cluster(_) => {
                bail!("Cluster is only implemented for Zisk, got {zkvm}")
            }
            ProverResource::Network(_) => unreachable!(),
        };
        instances.push(instance);
    }
    Ok(instances)
}

async fn load_compiled(
    el: ExecutionClient,
    zkvm: zkVMKind,
    guest_source: &GuestProgramSource,
) -> Result<CompiledGuest> {
    el.validate_zkvm(zkvm)?;
    let stateless_validator_kind = el.registered_kind()?;
    let guest_name = guest_artifact_name(stateless_validator_kind, zkvm);
    if let GuestProgramSource::LocalPath(path) = guest_source {
        let elf = fs::read(path.join(format!("{guest_name}.elf")))
            .with_context(|| format!("Failed to read ELF from path: {}", path.display()))?;
        let program_vk = fs::read(path.join(format!("{guest_name}.vk")))
            .with_context(|| format!("Failed to read program vk from path: {}", path.display()))?;
        let profiling_elf = fs::read(path.join(format!("{guest_name}-profiling.elf"))).ok();
        return Ok(CompiledGuest {
            elf,
            program_vk,
            profiling_elf,
        });
    }

    if let GuestProgramSource::ArtifactBaseUrl(base_url) = guest_source {
        return load_compiled_from_artifact_base_url(&guest_name, base_url).await;
    }

    let downloader = guest_downloader().await?;
    downloader
        .download(stateless_validator_kind, zkvm)
        .await
        .with_context(|| format!("Failed to download guest program: {guest_name}"))
}

/// Returns the released artifact name of the `stateless_validator` guest compiled for `zkvm`.
fn guest_artifact_name(stateless_validator: StatelessValidatorKind, zkvm: zkVMKind) -> String {
    format!(
        "stateless-validator-{stateless_validator}-{zkvm}-{}",
        zkvm.sdk_version()
    )
}

async fn guest_downloader() -> Result<Downloader> {
    match ERE_GUESTS_DOWNLOAD_KIND {
        "tag" => {
            info!(
                "Downloading guest programs from ere-guests release {}",
                ERE_GUESTS_DOWNLOAD_VALUE
            );
            Downloader::from_tag(ERE_GUESTS_DOWNLOAD_VALUE)
                .await
                .with_context(|| {
                    format!(
                        "Failed to create guest program downloader for ere-guests release {}",
                        ERE_GUESTS_DOWNLOAD_VALUE
                    )
                })
        }
        "commit" => {
            let github_token = env::var("GITHUB_TOKEN")
                .or_else(|_| env::var("GH_TOKEN"))
                .with_context(|| {
                    format!(
                        "GITHUB_TOKEN or GH_TOKEN must be set to download guest artifacts for ere-guests commit {}",
                        ERE_GUESTS_DOWNLOAD_VALUE
                    )
                })?;

            info!(
                "Downloading guest programs from ere-guests workflow artifacts for commit {}",
                ERE_GUESTS_DOWNLOAD_VALUE
            );
            Downloader::from_commit(ERE_GUESTS_DOWNLOAD_VALUE, &github_token)
                .await
                .with_context(|| {
                    format!(
                        "Failed to create guest program downloader for ere-guests commit {}",
                        ERE_GUESTS_DOWNLOAD_VALUE
                    )
                })
        }
        other => Err(anyhow!(
            "Unsupported ere-guests download source `{}` with value `{}`",
            other,
            ERE_GUESTS_DOWNLOAD_VALUE
        )),
    }
}

async fn load_compiled_from_artifact_base_url(
    guest_name: &str,
    base_url: &str,
) -> Result<CompiledGuest> {
    let client = reqwest::Client::new();
    let elf_url = guest_artifact_url(base_url, &format!("{guest_name}.elf"));
    let vk_url = guest_artifact_url(base_url, &format!("{guest_name}.vk"));
    let profiling_url = guest_artifact_url(base_url, &format!("{guest_name}-profiling.elf"));

    info!("Downloading guest program from {elf_url}");
    let elf = download_required_artifact(&client, &elf_url).await?;
    let program_vk = download_optional_artifact(&client, &vk_url)
        .await?
        .unwrap_or_default();
    let profiling_elf = download_optional_artifact(&client, &profiling_url).await?;

    Ok(CompiledGuest {
        elf,
        program_vk,
        profiling_elf,
    })
}

async fn download_required_artifact(client: &reqwest::Client, url: &str) -> Result<Vec<u8>> {
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to download required guest artifact from {url}"))?;
    let status = response.status();
    if !status.is_success() {
        bail!("Failed to download required guest artifact from {url}: HTTP {status}");
    }
    Ok(response
        .bytes()
        .await
        .with_context(|| format!("Failed to read required guest artifact from {url}"))?
        .to_vec())
}

async fn download_optional_artifact(
    client: &reqwest::Client,
    url: &str,
) -> Result<Option<Vec<u8>>> {
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("Skipping optional guest artifact {url}: {err}");
            return Ok(None);
        }
    };
    let status = response.status();
    if !status.is_success() {
        info!("Skipping optional guest artifact {url}: HTTP {status}");
        return Ok(None);
    }
    Ok(Some(
        response
            .bytes()
            .await
            .with_context(|| format!("Failed to read optional guest artifact from {url}"))?
            .to_vec(),
    ))
}

fn guest_artifact_url(base_url: &str, filename: &str) -> String {
    format!("{}/{}", base_url.trim_end_matches('/'), filename)
}

fn artifact_base_url_label(base_url: &str) -> Option<String> {
    base_url
        .split(['?', '#'])
        .next()
        .unwrap_or(base_url)
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|label| !label.is_empty())
        .map(std::string::ToString::to_string)
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
    proof: &EncodedProof,
    name: &str,
    zkvm_name: &str,
    proofs_folder: &Path,
    sub_folder: Option<&str>,
) -> Result<()> {
    let proof_dir = proofs_folder.join(sub_folder.unwrap_or("")).join(zkvm_name);

    fs::create_dir_all(&proof_dir)
        .with_context(|| format!("Failed to create directory: {}", proof_dir.display()))?;

    let proof_path = proof_dir.join(format!("{name}.proof"));
    fs::write(&proof_path, proof)
        .with_context(|| format!("Failed to write proof to {}", proof_path.display()))?;
    info!("Saved proof to {}", proof_path.display());

    Ok(())
}

fn public_output_matched(io: &impl GuestFixture, public_values: &[u8]) -> Result<bool> {
    let expected_public_values = io.expected_public_values()?;

    if public_values
        .split_at_checked(expected_public_values.len())
        .is_some_and(|(actual, trailing)| {
            actual == expected_public_values && trailing.iter().all(|byte| *byte == 0)
        })
    {
        Ok(true)
    } else {
        warn!(
            "Output mismatch for {}: Public values mismatch: expected {:?}, got {:?}",
            io.name(),
            expected_public_values,
            public_values
        );
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cost_context() -> CostEstimationContext {
        CostEstimationContext {
            execution_client: "reth".into(),
            execution_client_version: "0.1.0-rc.3".into(),
            zkvm: "sp1".into(),
            sdk_version: "v6.4.0".into(),
            ere_revision: "5023513".into(),
            elf_sha256: "elf".into(),
            input_sha256: "input".into(),
            estimator_settings: estimator_settings(zkVMKind::SP1, |_| None),
        }
    }

    #[test]
    fn estimator_preserves_costs_on_output_mismatch_and_handles_failures() -> Result<()> {
        let fixture = Fixture::new(vec![1, 2]);
        for (output, matched) in [(vec![1, 2, 0], true), (vec![1, 3], false)] {
            let metrics = collect_cost_estimation(&fixture, test_cost_context(), || {
                Ok((
                    output.as_slice().into(),
                    CostEstimation {
                        cost: BTreeMap::from([("opcode".into(), 99)]),
                        peak_heap_bytes: None,
                    },
                ))
            })?;
            let value = serde_json::to_value(metrics)?;
            assert_eq!(value["success"]["output_matched"], matched);
            assert_eq!(value["success"]["cost"]["opcode"], 99);
            assert!(value["success"]["peak_heap_bytes"].is_null());
        }
        for metrics in [
            collect_cost_estimation(&fixture, test_cost_context(), || bail!("estimator error"))?,
            collect_cost_estimation(&fixture, test_cost_context(), || panic!("estimator panic"))?,
        ] {
            assert!(matches!(metrics, CostEstimationMetrics::Crashed(_)));
        }
        Ok(())
    }

    #[test]
    fn estimator_settings_record_effective_defaults_and_overrides() {
        assert_eq!(
            estimator_settings(zkVMKind::OpenVM, |_| None)["segment_memory_bytes"],
            (29_usize << 29).to_string()
        );
        assert_eq!(
            estimator_settings(zkVMKind::OpenVM, |_| Some("invalid".into()))
                ["segment_memory_bytes"],
            (29_usize << 29).to_string()
        );
        let settings = estimator_settings(zkVMKind::Zisk, |key| Some(key.to_lowercase()));
        assert_eq!(settings["heap_start"], "ere_cost_estimation_heap_start");
        assert_eq!(settings["heap_end"], "ere_cost_estimation_heap_end");
    }

    #[test]
    fn action_skip_checks_payload_and_validates_even_forced_updates() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("fixture.json");
        assert!(!should_skip_action(&path, Action::Execute, false)?);
        for (field, action) in [
            ("execution", Action::Execute),
            ("proving", Action::Prove),
            ("verification", Action::Verify),
            ("cost_estimation", Action::EstimateCost),
        ] {
            fs::write(
                &path,
                serde_json::to_vec(&serde_json::json!({
                    "name": "fixture", "metadata": {}, "timestamp_completed": "2026-09-10T00:00:00Z",
                    (field): {"crashed": {"reason": "already attempted"}}
                }))?,
            )?;
            for selected in [
                Action::Execute,
                Action::Prove,
                Action::Verify,
                Action::EstimateCost,
            ] {
                assert_eq!(
                    should_skip_action(&path, selected, false)?,
                    selected == action
                );
                assert!(!should_skip_action(&path, selected, true)?);
            }
        }
        fs::write(&path, "invalid")?;
        assert!(should_skip_action(&path, Action::EstimateCost, true).is_err());
        Ok(())
    }

    struct Fixture {
        name: &'static str,
        expected_public_values: Vec<u8>,
    }

    impl Fixture {
        fn new(expected_public_values: Vec<u8>) -> Self {
            Self {
                name: "fixture",
                expected_public_values,
            }
        }
    }

    impl GuestFixture for Fixture {
        fn name(&self) -> String {
            self.name.to_string()
        }

        fn metadata(&self) -> serde_json::Value {
            serde_json::json!({})
        }

        fn input(&self) -> Result<Input> {
            Ok(Input::new())
        }

        fn expected_public_values(&self) -> Result<Vec<u8>> {
            Ok(self.expected_public_values.clone())
        }
    }

    #[test]
    fn public_output_matched_returns_true_for_matching_values() -> Result<()> {
        let fixture = Fixture::new(vec![1, 2, 3]);

        assert!(public_output_matched(&fixture, &[1, 2, 3],)?);

        Ok(())
    }

    #[test]
    fn public_output_matched_returns_false_for_mismatched_values() -> Result<()> {
        let fixture = Fixture::new(vec![1, 2, 3]);

        assert!(!public_output_matched(&fixture, &[1, 2, 4],)?);

        Ok(())
    }

    #[test]
    fn public_output_matched_accepts_zero_padding() -> Result<()> {
        let fixture = Fixture::new(vec![0xab]);

        let mut padded_public_values = vec![0xab];
        padded_public_values.resize(256, 0);

        assert!(public_output_matched(&fixture, &padded_public_values,)?);

        Ok(())
    }

    #[test]
    fn public_output_matched_rejects_nonzero_trailing_bytes() -> Result<()> {
        let fixture = Fixture::new(vec![0xab]);

        assert!(!public_output_matched(&fixture, &[0xab, 0x01])?);

        Ok(())
    }

    #[test]
    fn public_output_matched_rejects_truncated_output() -> Result<()> {
        let fixture = Fixture::new(vec![0xab, 0xcd]);

        assert!(!public_output_matched(&fixture, &[0xab])?);

        Ok(())
    }

    #[test]
    fn guest_artifact_url_joins_base_and_filename() {
        assert_eq!(
            guest_artifact_url(
                "https://github.com/paradigmxyz/stateless/releases/download/reth-guest-v0.1.0-rc.2/",
                "stateless-validator-reth-zisk-v1.1.0-alpha.elf",
            ),
            "https://github.com/paradigmxyz/stateless/releases/download/reth-guest-v0.1.0-rc.2/stateless-validator-reth-zisk-v1.1.0-alpha.elf"
        );
    }

    #[test]
    fn artifact_base_url_label_uses_last_path_segment() {
        assert_eq!(
            artifact_base_url_label(
                "https://github.com/paradigmxyz/stateless/releases/download/reth-guest-v0.1.0-rc.2/"
            )
            .as_deref(),
            Some("reth-guest-v0.1.0-rc.2")
        );
    }

    #[test]
    fn url_artifact_loader_requires_elf_but_not_vk_or_profiling_elf() -> Result<()> {
        let elf_path = format!(
            "/{}.elf",
            guest_artifact_name(StatelessValidatorKind::Reth, zkVMKind::Zisk)
        );
        let server =
            TestServer::spawn(move |path| (path == elf_path).then(|| Vec::from("elf-bytes")));

        let compiled = block_on(load_compiled(
            ExecutionClient::Reth,
            zkVMKind::Zisk,
            &GuestProgramSource::ArtifactBaseUrl(server.base_url()),
        ))?;

        assert_eq!(compiled.elf, b"elf-bytes");
        assert!(compiled.program_vk.is_empty());
        assert!(compiled.profiling_elf.is_none());

        Ok(())
    }

    #[test]
    fn url_artifact_loader_fails_when_elf_is_missing() {
        let server = TestServer::spawn(|_| None);

        let err = block_on(load_compiled(
            ExecutionClient::Reth,
            zkVMKind::Zisk,
            &GuestProgramSource::ArtifactBaseUrl(server.base_url()),
        ))
        .unwrap_err();

        assert!(err.to_string().contains(&format!(
            "{}.elf",
            guest_artifact_name(StatelessValidatorKind::Reth, zkVMKind::Zisk)
        )));
    }

    struct TestServer {
        base_url: String,
    }

    impl TestServer {
        fn spawn<F>(handler: F) -> Self
        where
            F: Fn(&str) -> Option<Vec<u8>> + Send + Sync + 'static,
        {
            use std::{
                io::{Read, Write},
                net::TcpListener,
                sync::Arc,
                thread,
            };

            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let base_url = format!("http://{}", listener.local_addr().unwrap());
            let handler = Arc::new(handler);

            thread::spawn(move || {
                for stream in listener.incoming().take(3) {
                    let Ok(mut stream) = stream else {
                        continue;
                    };
                    let mut request = [0_u8; 1024];
                    let Ok(read) = stream.read(&mut request) else {
                        continue;
                    };
                    let request = String::from_utf8_lossy(&request[..read]);
                    let path = request
                        .lines()
                        .next()
                        .and_then(|line| line.split_whitespace().nth(1))
                        .unwrap_or("/");

                    if let Some(body) = handler(path) {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                            body.len()
                        );
                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.write_all(&body);
                    } else {
                        let _ = stream.write_all(
                            b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        );
                    }
                }
            });

            Self { base_url }
        }

        fn base_url(&self) -> String {
            self.base_url.clone()
        }
    }
}
