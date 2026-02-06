//! Zisk profiling support for benchmark runs

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use tracing::info;

/// Configuration for Zisk profiling.
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    /// Output folder for Zisk profile results
    pub output_folder: PathBuf,
    /// ERE tag for docker image
    pub ere_tag: String,
}

impl ProfileConfig {
    /// Creates a new `ZiskProfileConfig` by computing the ERE tag from cargo tree
    pub fn new(output_folder: PathBuf) -> Result<Self> {
        let ere_tag = compute_ere_tag()?;
        Ok(Self {
            output_folder,
            ere_tag,
        })
    }
}

/// Computes the ERE tag by parsing `cargo tree -p ere-platform-trait` output
fn compute_ere_tag() -> Result<String> {
    let output = Command::new("cargo")
        .args(["tree", "-p", "ere-platform-trait"])
        .output()
        .context("Failed to execute cargo tree")?;

    if !output.status.success() {
        anyhow::bail!(
            "cargo tree failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let first_line = stdout.lines().next().context("No output from cargo tree")?;

    let commit = first_line
        .find('#')
        .map(|i| &first_line[i + 1..])
        .and_then(|s| s.get(..7))
        .context("Failed to extract commit hash from cargo tree output")?;

    Ok(commit.to_string())
}

/// Runs Zisk profiling for a single fixture.
pub fn run_profiling(
    config: &ProfileConfig,
    elf: &[u8],
    stdin: &[u8],
    fixture_name: &str,
    sub_folder: Option<&str>,
) -> Result<()> {
    info!("Running Zisk profiling for {}", fixture_name);

    let temp_dir = tempfile::tempdir().context("Failed to create temp directory for profiling")?;
    let temp_path = temp_dir.path();

    let input_path = temp_path.join("input.bin");
    let elf_path = temp_path.join("program.elf");
    fs::write(&input_path, stdin)
        .with_context(|| format!("Failed to write input.bin to {}", input_path.display()))?;
    fs::write(&elf_path, elf)
        .with_context(|| format!("Failed to write program.elf to {}", elf_path.display()))?;

    let docker_image = match env::var("ERE_IMAGE_REGISTRY") {
        Ok(registry) => format!("{}/ere-server-zisk:{}", registry, config.ere_tag),
        Err(_) => format!("ere-server-zisk:{}", config.ere_tag),
    };
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

    let profile_dir = config.output_folder.join(sub_folder.unwrap_or(""));
    fs::create_dir_all(&profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory: {}",
            profile_dir.display()
        )
    })?;

    let profile_path = profile_dir.join(format!("zisk_profile_{}.prof", fixture_name));
    fs::write(&profile_path, &output.stdout)
        .with_context(|| format!("Failed to write profile to {}", profile_path.display()))?;

    info!("Saved Zisk profile to {}", profile_path.display());

    Ok(())
}
