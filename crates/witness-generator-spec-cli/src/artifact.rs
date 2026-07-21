use std::{
    fs::{self, OpenOptions},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use alloy_primitives::hex;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use witness_generator_spec_cli::GeneratedInput;

const ARTIFACT_SCHEMA_VERSION: u64 = 1;
const ZSTD_LEVEL: i32 = 3;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct StatelessInputArtifact {
    pub(crate) schema_version: u64,
    pub(crate) network: String,
    pub(crate) chain_id: u64,
    pub(crate) block_number: u64,
    pub(crate) block_hash: String,
    pub(crate) slot_number: u64,
    pub(crate) collection_mode: String,
    pub(crate) collected_at: String,
    pub(crate) generator_package: String,
    pub(crate) generator_version: String,
    pub(crate) generator_git_commit: String,
    pub(crate) stateless_input_schema_id: String,
    pub(crate) stateless_input_byte_length: usize,
    pub(crate) stateless_input_sha256: String,
    pub(crate) stateless_input_bytes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtifactIndexEntry {
    pub(crate) schema_version: u64,
    pub(crate) network: String,
    pub(crate) chain_id: u64,
    pub(crate) block_number: u64,
    pub(crate) block_hash: String,
    pub(crate) slot_number: u64,
    pub(crate) collection_mode: String,
    pub(crate) collected_at: String,
    pub(crate) stateless_input_byte_length: usize,
    pub(crate) stateless_input_sha256: String,
    pub(crate) path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArtifactWriteResult {
    pub(crate) path: PathBuf,
    pub(crate) relative_path: PathBuf,
    pub(crate) created: bool,
}

impl StatelessInputArtifact {
    pub(crate) fn from_generated(
        network: &str,
        collection_mode: &str,
        generated: &GeneratedInput,
    ) -> anyhow::Result<Self> {
        Self::from_generated_at(
            network,
            collection_mode,
            generated,
            &utc_now_rfc3339()?,
            generator_git_commit(),
        )
    }

    pub(crate) fn from_generated_at(
        network: &str,
        collection_mode: &str,
        generated: &GeneratedInput,
        collected_at: &str,
        generator_git_commit: String,
    ) -> anyhow::Result<Self> {
        let schema_id = generated
            .bytes
            .get(..2)
            .context("generated stateless input is missing its two-byte schema id")?;

        Ok(Self {
            schema_version: ARTIFACT_SCHEMA_VERSION,
            network: network.to_owned(),
            chain_id: generated.chain_id,
            block_number: generated.block_number,
            block_hash: generated.block_hash.to_string(),
            slot_number: generated.slot_number,
            collection_mode: collection_mode.to_owned(),
            collected_at: collected_at.to_owned(),
            generator_package: env!("CARGO_PKG_NAME").to_owned(),
            generator_version: env!("CARGO_PKG_VERSION").to_owned(),
            generator_git_commit,
            stateless_input_schema_id: format!("0x{}", hex::encode(schema_id)),
            stateless_input_byte_length: generated.bytes.len(),
            stateless_input_sha256: sha256_hex(&generated.bytes),
            stateless_input_bytes: format!("0x{}", hex::encode(&generated.bytes)),
        })
    }

    pub(crate) fn index_entry(&self, path: &Path) -> ArtifactIndexEntry {
        ArtifactIndexEntry {
            schema_version: self.schema_version,
            network: self.network.clone(),
            chain_id: self.chain_id,
            block_number: self.block_number,
            block_hash: self.block_hash.clone(),
            slot_number: self.slot_number,
            collection_mode: self.collection_mode.clone(),
            collected_at: self.collected_at.clone(),
            stateless_input_byte_length: self.stateless_input_byte_length,
            stateless_input_sha256: self.stateless_input_sha256.clone(),
            path: path_to_slash_string(path),
        }
    }
}

pub(crate) fn write_artifact_atomic(
    blocks_root: &Path,
    artifact: &StatelessInputArtifact,
) -> anyhow::Result<ArtifactWriteResult> {
    let relative_path = relative_artifact_path(artifact);
    let path = blocks_root.join(&relative_path);
    if path.exists() {
        return Ok(ArtifactWriteResult {
            path,
            relative_path,
            created: false,
        });
    }

    let json = serde_json::to_vec_pretty(artifact).context("failed to serialize artifact JSON")?;
    let compressed =
        zstd::bulk::compress(&json, ZSTD_LEVEL).context("failed to compress artifact")?;
    write_bytes_atomic(&path, &compressed)?;

    Ok(ArtifactWriteResult {
        path,
        relative_path,
        created: true,
    })
}

pub(crate) fn read_artifact(path: &Path) -> anyhow::Result<StatelessInputArtifact> {
    let file = fs::File::open(path)
        .with_context(|| format!("failed to open artifact {}", path.display()))?;
    let json = zstd::stream::decode_all(file)
        .with_context(|| format!("failed to decompress artifact {}", path.display()))?;
    serde_json::from_slice(&json)
        .with_context(|| format!("failed to decode artifact JSON {}", path.display()))
}

pub(crate) fn append_index_entry(
    index_path: &Path,
    entry: &ArtifactIndexEntry,
) -> anyhow::Result<()> {
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create index directory {}", parent.display()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(index_path)
        .with_context(|| format!("failed to open index {}", index_path.display()))?;
    serde_json::to_writer(&mut file, entry).context("failed to serialize index entry")?;
    file.write_all(b"\n")
        .context("failed to write index entry")?;
    Ok(())
}

pub(crate) fn write_json_atomic<T>(path: &Path, value: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let bytes = serde_json::to_vec_pretty(value).context("failed to serialize JSON")?;
    write_bytes_atomic(path, &bytes)
}

pub(crate) fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }
    let part_path = part_path(path);
    fs::write(&part_path, bytes)
        .with_context(|| format!("failed to write partial file {}", part_path.display()))?;
    fs::rename(&part_path, path).with_context(|| {
        format!(
            "failed to atomically rename {} to {}",
            part_path.display(),
            path.display()
        )
    })?;
    Ok(())
}

pub(crate) fn relative_artifact_path(artifact: &StatelessInputArtifact) -> PathBuf {
    let chunk = artifact.block_number / 1_000;
    PathBuf::from(format!("{chunk:06}")).join(format!(
        "{}-{}.json.zst",
        artifact.block_number,
        filename_hash(&artifact.block_hash)
    ))
}

pub(crate) fn path_to_slash_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    format!("0x{}", hex::encode(Sha256::digest(bytes)))
}

pub(crate) fn file_sha256_hex(path: &Path) -> anyhow::Result<String> {
    let file =
        fs::File::open(path).with_context(|| format!("failed to open {}", path.display()))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let bytes_read = reader
            .read(&mut buffer)
            .with_context(|| format!("failed to read {}", path.display()))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("0x{}", hex::encode(hasher.finalize())))
}

pub(crate) fn utc_now_rfc3339() -> anyhow::Result<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("failed to format current timestamp")
}

fn part_path(path: &Path) -> PathBuf {
    path.with_extension(
        match path.extension().and_then(|extension| extension.to_str()) {
            Some(extension) => format!("{extension}.part"),
            None => "part".to_owned(),
        },
    )
}

fn filename_hash(hash: &str) -> &str {
    hash.strip_prefix("0x").unwrap_or(hash)
}

fn generator_git_commit() -> String {
    std::env::var("WITNESS_GENERATOR_SPEC_GIT_COMMIT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| option_env!("GIT_COMMIT").map(str::to_owned))
        .unwrap_or_else(|| "unknown".to_owned())
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;

    use super::*;

    #[test]
    fn artifact_serializes_and_roundtrips_through_zstd() {
        let dir = temp_dir("artifact_roundtrip");
        let generated = generated_input(42, B256::repeat_byte(0xaa));
        let artifact = StatelessInputArtifact::from_generated_at(
            "glamsterdam-devnet-5",
            "head",
            &generated,
            "2026-06-11T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap();

        let result = write_artifact_atomic(&dir, &artifact).unwrap();
        let decoded = read_artifact(&result.path).unwrap();

        assert!(result.created);
        assert_eq!(decoded, artifact);
        assert_eq!(decoded.stateless_input_byte_length, generated.bytes.len());
        assert_eq!(decoded.stateless_input_sha256, sha256_hex(&generated.bytes));
        assert_eq!(decoded.stateless_input_schema_id, "0x1501");
        assert_eq!(decoded.stateless_input_bytes, "0x15010203");
        assert_eq!(decoded.collection_mode, "head");

        let file = fs::File::open(&result.path).unwrap();
        let json = zstd::stream::decode_all(file).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(value["collectionMode"], "head");
        assert!(value.get("selector").is_none());
        assert!(value.get("canonicality").is_none());
    }

    #[test]
    fn artifact_paths_include_block_number_and_hash() {
        let first = StatelessInputArtifact::from_generated_at(
            "glamsterdam-devnet-5",
            "head",
            &generated_input(2_381, B256::repeat_byte(0xaa)),
            "2026-06-11T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap();
        let second = StatelessInputArtifact::from_generated_at(
            "glamsterdam-devnet-5",
            "head",
            &generated_input(2_381, B256::repeat_byte(0xbb)),
            "2026-06-11T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap();

        assert_ne!(
            relative_artifact_path(&first),
            relative_artifact_path(&second)
        );
        assert_eq!(
            relative_artifact_path(&first),
            PathBuf::from("000002").join(format!("2381-{}.json.zst", "aa".repeat(32)))
        );
    }

    #[test]
    fn artifact_rejects_generated_input_without_schema_id() {
        let mut generated = generated_input(42, B256::repeat_byte(0xaa));
        generated.bytes = vec![0x15];

        let error = StatelessInputArtifact::from_generated_at(
            "glamsterdam-devnet-7",
            "head",
            &generated,
            "2026-06-11T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap_err();

        assert!(error.to_string().contains("two-byte schema id"));
    }

    fn generated_input(block_number: u64, block_hash: B256) -> GeneratedInput {
        GeneratedInput {
            bytes: vec![0x15, 0x01, 0x02, 0x03],
            block_hash,
            block_number,
            slot_number: 64,
            chain_id: 1,
        }
    }

    fn temp_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "witness-generator-spec-cli-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }
}
