//! Proof verification from disk or remote URL

use anyhow::{anyhow, Context, Result};
use ere_dockerized::DockerizedzkVM;
use ere_zkvm_interface::{zkVM, ProofKind};
use std::fs;
use std::panic;
use std::path::{Path, PathBuf};
use tracing::info;
use zkevm_metrics::{BenchmarkRun, CrashInfo, HardwareInfo, VerificationMetrics};

use crate::runner::{get_panic_msg, RunConfig};

/// Loads proof artifacts from disk and verifies them using the given zkVM.
pub fn run_verify_from_disk(
    zkvm: &DockerizedzkVM,
    config: &RunConfig,
    proofs_folder: &Path,
) -> Result<()> {
    HardwareInfo::detect().to_path(config.output_folder.join("hardware.json"))?;

    let zkvm_name = format!("{}-v{}", zkvm.name(), zkvm.sdk_version());
    let proof_dir = proofs_folder
        .join(config.sub_folder.as_deref().unwrap_or(""))
        .join(&zkvm_name);

    if !proof_dir.exists() {
        info!("No proofs found for {zkvm_name} at {}", proof_dir.display());
        return Ok(());
    }

    let proof_entries: Vec<_> = walkdir::WalkDir::new(&proof_dir)
        .min_depth(1)
        .max_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "proof")
        })
        .collect();

    // Warmup pass: verify the first proof to warm up the zkVM setup (if any).
    if let Some(first) = proof_entries.first() {
        info!(
            "Warmup: verifying {} (result will be discarded)",
            first.path().display()
        );
        let proof_bytes = fs::read(first.path())
            .with_context(|| format!("Failed to read proof from {}", first.path().display()))?;
        let proof = ere_zkvm_interface::Proof::new(ProofKind::Compressed, proof_bytes);
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.verify(&proof)));
        info!("Warmup complete");
    }

    for entry in &proof_entries {
        let fixture_name = entry
            .path()
            .file_stem()
            .ok_or_else(|| anyhow!("Invalid proof file name: {}", entry.path().display()))?
            .to_string_lossy()
            .to_string();

        let out_path = config
            .output_folder
            .join(config.sub_folder.as_deref().unwrap_or(""))
            .join(format!("{zkvm_name}/{fixture_name}.json"));

        if !config.force_rerun && out_path.exists() {
            info!("Skipping {fixture_name} (already exists)");
            continue;
        }

        info!("Verifying proof for {fixture_name}");

        let proof_bytes = fs::read(entry.path())
            .with_context(|| format!("Failed to read proof from {}", entry.path().display()))?;
        // NOTE: save_proof writes raw bytes from proof.as_bytes(), which strips the
        // ProofKind discriminant. We reconstruct as Compressed here because the prove path
        // always uses ProofKind::Compressed. If other proof kinds are added, the file
        // format should be extended to store the proof kind.
        let proof = ere_zkvm_interface::Proof::new(ProofKind::Compressed, proof_bytes);

        let verify_start = std::time::Instant::now();
        let verification_result =
            panic::catch_unwind(panic::AssertUnwindSafe(|| zkvm.verify(&proof)));

        let verification = match verification_result {
            Ok(Ok(_public_values)) => {
                let verification_time_ms = verify_start.elapsed().as_millis();
                VerificationMetrics::Success {
                    proof_size: proof.as_bytes().len(),
                    verification_time_ms,
                }
            }
            Ok(Err(e)) => VerificationMetrics::Crashed(CrashInfo {
                reason: e.to_string(),
            }),
            Err(panic_info) => VerificationMetrics::Crashed(CrashInfo {
                reason: get_panic_msg(panic_info),
            }),
        };

        let report = BenchmarkRun {
            name: fixture_name,
            timestamp_completed: zkevm_metrics::chrono::Utc::now(),
            metadata: serde_json::Value::Null,
            execution: None,
            proving: None,
            verification: Some(verification),
        };

        info!("Saving verification report");
        report.to_path(out_path)?;
    }

    Ok(())
}

/// Downloads a `.tar.gz` archive from a URL and extracts it to a temporary directory.
pub async fn download_and_extract_proofs(url: &str) -> Result<tempfile::TempDir> {
    info!("Downloading proofs archive from {url}");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to send HTTP request for proofs archive")?
        .error_for_status()
        .context("HTTP error downloading proofs archive")?;

    let bytes = response
        .bytes()
        .await
        .context("Failed to read proofs archive response body")?;

    info!("Downloaded {} bytes, extracting...", bytes.len());
    let tmp = tempfile::tempdir().context("Failed to create temporary directory")?;

    let decoder = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(tmp.path())
        .context("Failed to extract proofs .tar.gz archive")?;

    info!("Extracted proofs to {}", tmp.path().display());
    Ok(tmp)
}

/// If the extracted archive contains a single top-level directory, return that
/// directory as the proofs root. Otherwise, return the extraction directory itself.
pub fn resolve_extracted_root(extracted: &Path) -> Result<PathBuf> {
    let entries: Vec<_> = fs::read_dir(extracted)
        .context("Failed to read extracted proofs directory")?
        .filter_map(|e| e.ok())
        .collect();

    if entries.len() == 1 && entries[0].file_type().is_ok_and(|ft| ft.is_dir()) {
        Ok(entries[0].path())
    } else {
        Ok(extracted.to_path_buf())
    }
}
