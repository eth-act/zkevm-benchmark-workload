use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tar::{Builder, Header};

use crate::{
    artifact::{
        self, StatelessInputArtifact, file_sha256_hex, path_to_slash_string, read_artifact,
    },
    config::CollectorConfig,
};

const ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone)]
struct ArtifactFile {
    path: PathBuf,
    relative_path: PathBuf,
    artifact: StatelessInputArtifact,
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
    slot_number: u64,
    chain_id: u64,
    stateless_input_sha256: String,
    stateless_input_byte_length: usize,
    compressed_file_sha256: String,
    compressed_file_byte_length: u64,
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
        let artifact = read_artifact(&path)?;
        let relative_path = path
            .strip_prefix(config.blocks_root())
            .with_context(|| {
                format!(
                    "artifact {} is not under {}",
                    path.display(),
                    config.blocks_root().display()
                )
            })?
            .to_path_buf();
        by_block
            .entry(artifact.block_number)
            .or_default()
            .push(ArtifactFile {
                path,
                relative_path,
                artifact,
            });
    }

    for files in by_block.values_mut() {
        files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
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
        let archive_path = PathBuf::from("blocks").join(&artifact.relative_path);
        tar.append_path_with_name(&artifact.path, &archive_path)
            .with_context(|| {
                format!(
                    "failed to append {} as {}",
                    artifact.path.display(),
                    archive_path.display()
                )
            })?;
    }

    let manifest_bytes =
        serde_json::to_vec_pretty(&manifest).context("failed to serialize batch manifest")?;
    let mut header = Header::new_gnu();
    header.set_size(manifest_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    tar.append_data(&mut header, "manifest.json", manifest_bytes.as_slice())
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
            let archive_path = PathBuf::from("blocks").join(&file.relative_path);
            Ok(BatchManifestArtifact {
                archive_path: path_to_slash_string(&archive_path),
                block_number: file.artifact.block_number,
                block_hash: file.artifact.block_hash.clone(),
                slot_number: file.artifact.slot_number,
                chain_id: file.artifact.chain_id,
                stateless_input_sha256: file.artifact.stateless_input_sha256.clone(),
                stateless_input_byte_length: file.artifact.stateless_input_byte_length,
                compressed_file_sha256: file_sha256_hex(&file.path)?,
                compressed_file_byte_length: fs::metadata(&file.path)
                    .with_context(|| format!("failed to stat {}", file.path.display()))?
                    .len(),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(BatchManifest {
        schema_version: 1,
        network: config.network.clone(),
        batch_start_block: start,
        batch_end_block: end,
        batch_size: config.batch_size,
        artifact_count: artifacts.len(),
        created_at: artifact::utc_now_rfc3339()?,
        artifacts,
    })
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
    use tar::Archive;
    use witness_generator_spec_cli::GeneratedInput;

    use crate::artifact::{StatelessInputArtifact, write_artifact_atomic};

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
        assert!(
            manifest
                .artifacts
                .iter()
                .all(|artifact| artifact.archive_path.starts_with("blocks/"))
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

    fn write_generated_artifact(config: &CollectorConfig, block_number: u64, block_hash: B256) {
        let generated = GeneratedInput {
            bytes: vec![0x00, 0x01, block_number as u8],
            block_hash,
            block_number,
            slot_number: block_number + 100,
            chain_id: 1,
        };
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
            if entry.path().unwrap().as_ref() == Path::new("manifest.json") {
                let mut bytes = Vec::new();
                entry.read_to_end(&mut bytes).unwrap();
                return serde_json::from_slice(&bytes).unwrap();
            }
        }
        panic!("manifest not found");
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
