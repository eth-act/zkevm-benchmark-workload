use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tar::{Builder, Header};

use crate::{
    artifact::{
        self, BATCH_MANIFEST_PATH, StatelessInputArtifact, fixture_archive_path,
        path_to_slash_string, read_artifact_with_json, sha256_hex,
    },
    config::CollectorConfig,
};

const ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone)]
struct ArtifactFile {
    path: PathBuf,
    artifact: StatelessInputArtifact,
    fixture_json: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchManifest {
    schema_version: u64,
    network: String,
    batch_start_block: u64,
    batch_end_block: u64,
    batch_size: u64,
    artifact_count: usize,
    created_at: String,
    artifacts: Vec<BatchManifestArtifact>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchManifestArtifact {
    archive_path: String,
    block_number: u64,
    block_hash: String,
    gas_used: u64,
    slot_number: u64,
    chain_id: u64,
    stateless_input_byte_length: usize,
    fixture_sha256: String,
    fixture_byte_length: usize,
}

pub(crate) fn export_batches(
    config: &CollectorConfig,
    force: bool,
) -> anyhow::Result<Vec<PathBuf>> {
    let by_block = artifacts_by_block(config)?;
    let complete_batches = complete_batch_starts(&by_block, config.batch_size);
    let mut exported = Vec::new();

    fs::create_dir_all(config.batches_root()).with_context(|| {
        format!(
            "failed to create batch export directory {}",
            config.batches_root().display()
        )
    })?;

    for start in complete_batches {
        let end = start + config.batch_size - 1;
        let out_path = config.batches_root().join(format!("{start}-{end}.tar.zst"));
        if out_path.exists() && !force {
            continue;
        }

        let artifacts = batch_artifacts(&by_block, start, end);
        write_batch_archive(config, start, end, &artifacts, &out_path, force)?;
        exported.push(out_path);
    }

    Ok(exported)
}

fn artifacts_by_block(
    config: &CollectorConfig,
) -> anyhow::Result<BTreeMap<u64, Vec<ArtifactFile>>> {
    let mut files = Vec::new();
    collect_artifact_files(&config.blocks_root(), &mut files)?;
    files.sort();

    let mut by_block: BTreeMap<u64, Vec<ArtifactFile>> = BTreeMap::new();
    for path in files {
        let (artifact, fixture_json) = read_artifact_with_json(&path)?;
        by_block
            .entry(artifact.block_number)
            .or_default()
            .push(ArtifactFile {
                path,
                artifact,
                fixture_json,
            });
    }

    for files in by_block.values_mut() {
        files.sort_by(|left, right| left.path.cmp(&right.path));
    }

    Ok(by_block)
}

fn complete_batch_starts(by_block: &BTreeMap<u64, Vec<ArtifactFile>>, batch_size: u64) -> Vec<u64> {
    let starts = by_block
        .keys()
        .map(|block_number| (block_number / batch_size) * batch_size)
        .collect::<BTreeSet<_>>();

    starts
        .into_iter()
        .filter(|start| {
            let end = start + batch_size - 1;
            (*start..=end).all(|block_number| by_block.contains_key(&block_number))
        })
        .collect()
}

fn batch_artifacts(
    by_block: &BTreeMap<u64, Vec<ArtifactFile>>,
    start: u64,
    end: u64,
) -> Vec<ArtifactFile> {
    (start..=end)
        .flat_map(|block_number| by_block.get(&block_number).into_iter().flatten())
        .cloned()
        .collect()
}

fn write_batch_archive(
    config: &CollectorConfig,
    start: u64,
    end: u64,
    artifacts: &[ArtifactFile],
    out_path: &Path,
    force: bool,
) -> anyhow::Result<()> {
    let manifest = build_manifest(config, start, end, artifacts)?;
    let part_path = archive_part_path(out_path);
    if let Some(parent) = part_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    let file = fs::File::create(&part_path)
        .with_context(|| format!("failed to create partial archive {}", part_path.display()))?;
    let encoder = zstd::stream::write::Encoder::new(file, ZSTD_LEVEL)
        .context("failed to create zstd encoder")?;
    let mut tar = Builder::new(encoder);

    for artifact in artifacts {
        let archive_path = fixture_archive_path(&artifact.artifact);
        append_bytes(&mut tar, &archive_path, &artifact.fixture_json).with_context(|| {
            format!(
                "failed to append {} as {}",
                artifact.path.display(),
                archive_path.display()
            )
        })?;
    }

    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("failed to serialize batch manifest")?;
    append_bytes(&mut tar, Path::new(BATCH_MANIFEST_PATH), &manifest_bytes)
        .context("failed to append batch manifest")?;

    let encoder = tar.into_inner().context("failed to finish tar archive")?;
    encoder.finish().context("failed to finish zstd archive")?;

    if out_path.exists() && force {
        fs::remove_file(out_path)
            .with_context(|| format!("failed to replace archive {}", out_path.display()))?;
    }
    fs::rename(&part_path, out_path).with_context(|| {
        format!(
            "failed to atomically rename {} to {}",
            part_path.display(),
            out_path.display()
        )
    })?;

    Ok(())
}

fn build_manifest(
    config: &CollectorConfig,
    start: u64,
    end: u64,
    artifacts: &[ArtifactFile],
) -> anyhow::Result<BatchManifest> {
    let artifacts = artifacts
        .iter()
        .map(|file| {
            let archive_path = fixture_archive_path(&file.artifact);
            Ok(BatchManifestArtifact {
                archive_path: path_to_slash_string(&archive_path),
                block_number: file.artifact.block_number,
                block_hash: file.artifact.block_hash.clone(),
                gas_used: file.artifact.gas_used,
                slot_number: file.artifact.slot_number,
                chain_id: file.artifact.chain_id,
                stateless_input_byte_length: file.artifact.stateless_input_byte_length,
                fixture_sha256: sha256_hex(&file.fixture_json),
                fixture_byte_length: file.fixture_json.len(),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(BatchManifest {
        schema_version: 2,
        network: config.network.clone(),
        batch_start_block: start,
        batch_end_block: end,
        batch_size: config.batch_size,
        artifact_count: artifacts.len(),
        created_at: artifact::utc_now_rfc3339()?,
        artifacts,
    })
}

fn append_bytes<W: Write>(tar: &mut Builder<W>, path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    let mut header = Header::new_gnu();
    header.set_size(bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(&mut header, path, bytes)
        .context("failed to append tar entry")
}

fn collect_artifact_files(dir: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    if !dir.exists() {
        return Ok(());
    }
    for entry in
        fs::read_dir(dir).with_context(|| format!("failed to read directory {}", dir.display()))?
    {
        let entry = entry.with_context(|| format!("failed to read entry in {}", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to read file type for {}", path.display()))?;
        if file_type.is_dir() {
            collect_artifact_files(&path, files)?;
        } else if is_artifact_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn is_artifact_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".json.zst"))
}

fn archive_part_path(path: &Path) -> PathBuf {
    path.with_extension(
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(extension) => format!("{extension}.part"),
            None => "part".to_owned(),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use alloy_primitives::B256;
    use benchmark_runner::stateless_validator::{
        ExecutionClient, benchmark_fixture_paths, stateless_validator_input_iter,
    };
    use tar::Archive;

    use crate::artifact::{StatelessInputArtifact, test_generated_input, write_artifact_atomic};

    use super::*;

    #[test]
    fn exports_complete_batches_with_manifest() {
        let config = test_config("complete_batch", 2);
        write_generated_artifact(&config, 0, B256::repeat_byte(0xaa));
        write_generated_artifact(&config, 1, B256::repeat_byte(0xbb));
        write_generated_artifact(&config, 2, B256::repeat_byte(0xcc));

        let exported = export_batches(&config, false).unwrap();

        assert_eq!(exported.len(), 1);
        assert_eq!(
            exported[0].file_name().and_then(|name| name.to_str()),
            Some("0-1.tar.zst")
        );
        let manifest = read_manifest_from_archive(&exported[0]);
        assert_eq!(manifest.batch_start_block, 0);
        assert_eq!(manifest.batch_end_block, 1);
        assert_eq!(manifest.artifact_count, 2);
        assert_eq!(manifest.schema_version, 2);
        assert!(manifest.artifacts.iter().all(|artifact| {
            artifact.archive_path.starts_with("blockchain_tests/")
                && artifact.archive_path.ends_with(".json")
        }));
        let entries = archive_entries(&exported[0]);
        assert!(entries.contains(&BATCH_MANIFEST_PATH.to_owned()));
        assert_eq!(
            entries
                .iter()
                .filter(|path| path.starts_with("blockchain_tests/"))
                .count(),
            2
        );
    }

    #[test]
    fn skips_existing_batch_without_force() {
        let config = test_config("skip_existing", 2);
        write_generated_artifact(&config, 0, B256::repeat_byte(0xaa));
        write_generated_artifact(&config, 1, B256::repeat_byte(0xbb));

        assert_eq!(export_batches(&config, false).unwrap().len(), 1);
        assert!(export_batches(&config, false).unwrap().is_empty());
        assert_eq!(export_batches(&config, true).unwrap().len(), 1);
    }

    #[test]
    fn extracted_batch_is_directly_loadable_by_benchmark_runner() {
        let config = test_config("benchmark_ready", 2);
        write_generated_artifact(&config, 0, B256::repeat_byte(0xaa));
        write_generated_artifact(&config, 1, B256::repeat_byte(0xbb));
        let archive = export_batches(&config, false).unwrap().remove(0);
        let extracted = config.out_root.join("extracted");

        let file = fs::File::open(archive).unwrap();
        let decoder = zstd::stream::read::Decoder::new(file).unwrap();
        Archive::new(decoder).unpack(&extracted).unwrap();

        let paths = benchmark_fixture_paths(&extracted).unwrap();
        assert_eq!(paths.len(), 2);
        assert!(
            paths
                .iter()
                .all(|path| path.starts_with(extracted.join("blockchain_tests")))
        );
        let fixtures =
            stateless_validator_input_iter(&extracted, None, ExecutionClient::Reth, None)
                .unwrap()
                .collect::<anyhow::Result<Vec<_>>>()
                .unwrap();
        assert_eq!(fixtures.len(), 2);
        assert_eq!(
            fixtures[0].expected_public_values().unwrap(),
            test_generated_input(0, B256::repeat_byte(0xaa)).stateless_output_bytes
        );
        assert_eq!(fixtures[0].metadata()["block_used_gas"], 21_000);
    }

    fn write_generated_artifact(config: &CollectorConfig, block_number: u64, block_hash: B256) {
        let generated = test_generated_input(block_number, block_hash);
        let artifact = StatelessInputArtifact::from_generated_at(
            &config.network,
            "head",
            &generated,
            "2026-06-11T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap();
        write_artifact_atomic(&config.blocks_root(), &artifact).unwrap();
    }

    fn read_manifest_from_archive(path: &Path) -> BatchManifest {
        let file = fs::File::open(path).unwrap();
        let decoder = zstd::stream::read::Decoder::new(file).unwrap();
        let mut archive = Archive::new(decoder);
        for entry in archive.entries().unwrap() {
            let mut entry = entry.unwrap();
            if entry.path().unwrap().as_ref() == Path::new(BATCH_MANIFEST_PATH) {
                let mut bytes = Vec::new();
                entry.read_to_end(&mut bytes).unwrap();
                return serde_json::from_slice(&bytes).unwrap();
            }
        }
        panic!("manifest not found");
    }

    fn archive_entries(path: &Path) -> Vec<String> {
        let file = fs::File::open(path).unwrap();
        let decoder = zstd::stream::read::Decoder::new(file).unwrap();
        Archive::new(decoder)
            .entries()
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect()
    }

    fn test_config(name: &str, batch_size: u64) -> CollectorConfig {
        let out_root = std::env::temp_dir().join(format!(
            "witness-generator-spec-cli-export-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&out_root);
        CollectorConfig {
            network: "glamsterdam-devnet-5".to_owned(),
            cl_url: "http://cl".to_owned(),
            el_url: "http://el".to_owned(),
            out_root,
            poll_interval: std::time::Duration::from_secs(4),
            request_timeout: std::time::Duration::from_secs(30),
            batch_size,
            r2: None,
        }
    }
}
