//! Zisk profiling support for benchmark runs

use anyhow::{Context, Result};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};
use tracing::info;

/// ERE commit hash extracted from `Cargo.lock` at build time.
const DEFAULT_ERE_TAG: &str = ere_dockerized::DOCKER_IMAGE_TAG;

/// Configuration for Zisk profiling.
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    /// Output folder for Zisk profile results
    pub output_folder: PathBuf,
}

impl ProfileConfig {
    /// Creates a new `ProfileConfig`.
    pub const fn new(output_folder: PathBuf) -> Self {
        Self { output_folder }
    }
}

/// Outcome of a profiling attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileOutcome {
    /// Profiling succeeded and a `.prof` artifact was written.
    Success,
    /// Profiling failed and an `.error.txt` sidecar was written when possible.
    Failed(String),
}

#[derive(Debug)]
struct ProfileArtifacts {
    profile_dir: PathBuf,
    profile_path: PathBuf,
    error_path: PathBuf,
}

impl ProfileArtifacts {
    fn new(config: &ProfileConfig, fixture_name: &str, sub_folder: Option<&str>) -> Self {
        let profile_dir = config.output_folder.join(sub_folder.unwrap_or(""));
        let profile_path = profile_dir.join(format!("zisk_profile_{fixture_name}.prof"));
        let error_path = profile_dir.join(format!("zisk_profile_{fixture_name}.error.txt"));
        Self {
            profile_dir,
            profile_path,
            error_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProfilingCommandOutput {
    success: bool,
    status_code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

/// Runs Zisk profiling for a single fixture.
pub fn run_profiling(
    config: &ProfileConfig,
    elf: &[u8],
    stdin: &[u8],
    fixture_name: &str,
    sub_folder: Option<&str>,
) -> ProfileOutcome {
    run_profiling_with_runner(
        config,
        elf,
        stdin,
        fixture_name,
        sub_folder,
        run_ziskemu_command,
    )
}

fn run_profiling_with_runner<F>(
    config: &ProfileConfig,
    elf: &[u8],
    stdin: &[u8],
    fixture_name: &str,
    sub_folder: Option<&str>,
    command_runner: F,
) -> ProfileOutcome
where
    F: FnOnce(&Path) -> Result<ProfilingCommandOutput>,
{
    let artifacts = ProfileArtifacts::new(config, fixture_name, sub_folder);
    info!("Running Zisk profiling for {}", fixture_name);

    match try_run_profiling(elf, stdin, fixture_name, &artifacts, command_runner) {
        Ok(()) => ProfileOutcome::Success,
        Err(err) => record_profiling_failure(&artifacts, fixture_name, err),
    }
}

fn try_run_profiling<F>(
    elf: &[u8],
    stdin: &[u8],
    fixture_name: &str,
    artifacts: &ProfileArtifacts,
    command_runner: F,
) -> Result<()>
where
    F: FnOnce(&Path) -> Result<ProfilingCommandOutput>,
{
    let temp_dir = tempfile::tempdir().context("Failed to create temp directory for profiling")?;
    let temp_path = temp_dir.path();

    let input_path = temp_path.join("input.bin");
    let elf_path = temp_path.join("program.elf");
    fs::write(&input_path, length_prefixed_and_padded(stdin))
        .with_context(|| format!("Failed to write input.bin to {}", input_path.display()))?;
    fs::write(&elf_path, elf)
        .with_context(|| format!("Failed to write program.elf to {}", elf_path.display()))?;

    let output = command_runner(temp_path)?;

    if !output.success {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Zisk profiling failed for fixture '{}': docker command exited with code {}\nstderr: {}",
            fixture_name,
            output.status_code.unwrap_or(-1),
            stderr
        );
    }

    fs::create_dir_all(&artifacts.profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory: {}",
            artifacts.profile_dir.display()
        )
    })?;

    fs::write(&artifacts.profile_path, &output.stdout).with_context(|| {
        format!(
            "Failed to write profile to {}",
            artifacts.profile_path.display()
        )
    })?;

    remove_file_if_exists(&artifacts.error_path).with_context(|| {
        format!(
            "Failed to remove stale profiling error file {}",
            artifacts.error_path.display()
        )
    })?;

    info!("Saved Zisk profile to {}", artifacts.profile_path.display());

    Ok(())
}

fn run_ziskemu_command(temp_path: &Path) -> Result<ProfilingCommandOutput> {
    let registry_prefix = env::var("ERE_IMAGE_REGISTRY")
        .map(|r| format!("{r}/"))
        .unwrap_or_default();
    let docker_image = format!("{registry_prefix}ere-server-zisk:{DEFAULT_ERE_TAG}");
    let volume_mount = format!("{}:/data", temp_path.display());

    let output = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &volume_mount,
            "--entrypoint",
            "ziskemu",
            &docker_image,
            "-X",
            "-S",
            "-D",
            "-e",
            "/data/program.elf",
            "-i",
            "/data/input.bin",
        ])
        .output()
        .context("Failed to execute docker command for Zisk profiling")?;

    Ok(ProfilingCommandOutput {
        success: output.status.success(),
        status_code: output.status.code(),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

fn record_profiling_failure(
    artifacts: &ProfileArtifacts,
    fixture_name: &str,
    err: anyhow::Error,
) -> ProfileOutcome {
    let mut message = err.to_string();

    if let Err(remove_err) = remove_file_if_exists(&artifacts.profile_path) {
        message.push_str(&format!(
            "\nAdditionally failed to remove stale profile {}: {}",
            artifacts.profile_path.display(),
            remove_err
        ));
    }

    let sidecar = format!(
        "fixture: {fixture_name}\ntimestamp_utc: {}\nerror:\n{message}\n",
        zkevm_metrics::chrono::Utc::now().to_rfc3339()
    );

    match write_failure_sidecar(artifacts, &sidecar) {
        Ok(()) => ProfileOutcome::Failed(message),
        Err(write_err) => ProfileOutcome::Failed(format!(
            "{message}\nAdditionally failed to write profiling error artifact {}: {}",
            artifacts.error_path.display(),
            write_err
        )),
    }
}

fn write_failure_sidecar(artifacts: &ProfileArtifacts, sidecar: &str) -> Result<()> {
    fs::create_dir_all(&artifacts.profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory: {}",
            artifacts.profile_dir.display()
        )
    })?;
    fs::write(&artifacts.error_path, sidecar).with_context(|| {
        format!(
            "Failed to write profiling error file to {}",
            artifacts.error_path.display()
        )
    })?;
    info!(
        "Saved Zisk profiling error to {}",
        artifacts.error_path.display()
    );
    Ok(())
}

fn remove_file_if_exists(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err.into()),
    }
}

/// Mirrors `ere-zisk` input preparation for direct `ziskemu` execution.
fn length_prefixed_and_padded(data: &[u8]) -> Vec<u8> {
    let len = (8 + data.len()).next_multiple_of(8);
    let mut buf = Vec::with_capacity(len);
    buf.extend_from_slice(&(data.len() as u64).to_le_bytes());
    buf.extend_from_slice(data);
    buf.resize(len, 0);
    buf
}
