//! Zisk profiling support for benchmark runs

use anyhow::{Context, Result};
use std::path::PathBuf;
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

/// Runs Zisk profiling for a single fixture.
pub fn run_profiling(
    config: &ProfileConfig,
    elf: &[u8],
    stdin: &[u8],
    fixture_name: &str,
    sub_folder: Option<&str>,
) -> Result<()> {
    let profile_dir = config.output_folder.join(sub_folder.unwrap_or(""));
    let profile_path = profile_dir.join(format!("zisk_profile_{fixture_name}.prof"));
    info!("Running Zisk profiling for {}", fixture_name);

    let temp_dir = tempfile::tempdir().context("Failed to create temp directory for profiling")?;
    let temp_path = temp_dir.path();

    let input_path = temp_path.join("input.bin");
    let elf_path = temp_path.join("program.elf");
    fs::write(&input_path, stdin)
        .with_context(|| format!("Failed to write input.bin to {}", input_path.display()))?;
    fs::write(&elf_path, elf)
        .with_context(|| format!("Failed to write program.elf to {}", elf_path.display()))?;

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

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Zisk profiling failed for fixture '{}': docker command exited with code {}\nstderr: {}",
            fixture_name,
            output.status.code().unwrap_or(-1),
            stderr
        );
    }

    fs::create_dir_all(&profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory: {}",
            profile_dir.display()
        )
    })?;

    fs::write(&profile_path, &output.stdout)
        .with_context(|| format!("Failed to write profile to {}", profile_path.display()))?;

    info!("Saved Zisk profile to {}", profile_path.display());

    Ok(())
}
