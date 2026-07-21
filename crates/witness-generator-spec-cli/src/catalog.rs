use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{Context, ensure};
use serde::{Deserialize, Serialize};
use tar::Archive;

use crate::{
    artifact::{self, path_to_slash_string, write_bytes_atomic},
    config::CollectorConfig,
};

const CATALOG_SCHEMA_VERSION: u64 = 2;
const CATALOG_KIND: &str = "stateless-inputs-public-catalog";
const HTML_INDEX: &str = "index.html";
const PUBLIC_MANIFEST: &str = "manifest.json";
const PUBLIC_BATCHES_INDEX: &str = "batches.jsonl";
const CHECKSUMS: &str = "SHA256SUMS";
const BATCH_PREFIX: &str = "exports/batches";
const STALE_PUBLIC_BLOCKS_INDEX: &str = "blocks.jsonl";

pub(crate) const REQUIRED_CATALOG_FILES: &[&str] =
    &[HTML_INDEX, PUBLIC_MANIFEST, PUBLIC_BATCHES_INDEX, CHECKSUMS];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CatalogGeneration {
    pub(crate) artifact_count: usize,
    pub(crate) batch_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublicManifest {
    schema_version: u64,
    kind: String,
    network: String,
    generated_at: String,
    batch_size: u64,
    paths: PublicManifestPaths,
    batches: PublicBatchesSummary,
    notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublicManifestPaths {
    html: String,
    manifest: String,
    batches: String,
    checksums: String,
    batch_prefix: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublicBatchesSummary {
    count: usize,
    artifact_count: usize,
    first_start_block: Option<u64>,
    last_end_block: Option<u64>,
    total_byte_length: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublicBatchEntry {
    schema_version: u64,
    network: String,
    batch_start_block: u64,
    batch_end_block: u64,
    batch_size: u64,
    artifact_count: usize,
    created_at: String,
    byte_length: u64,
    sha256: String,
    path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BatchArchiveManifest {
    network: String,
    batch_start_block: u64,
    batch_end_block: u64,
    batch_size: u64,
    artifact_count: usize,
    created_at: String,
}

pub(crate) fn required_catalog_files(config: &CollectorConfig) -> Vec<(&'static str, PathBuf)> {
    REQUIRED_CATALOG_FILES
        .iter()
        .map(|name| (*name, config.network_root().join(name)))
        .collect()
}

pub(crate) fn generate_catalog(config: &CollectorConfig) -> anyhow::Result<CatalogGeneration> {
    let batches = read_batch_entries(config)?;
    let artifact_count = batches.iter().map(|batch| batch.artifact_count).sum();
    let manifest = public_manifest(config, &batches)?;

    write_json(config.network_root().join(PUBLIC_MANIFEST), &manifest)?;
    write_jsonl(config.network_root().join(PUBLIC_BATCHES_INDEX), &batches)?;
    write_bytes_atomic(
        &config.network_root().join(CHECKSUMS),
        checksums_file(&batches).as_bytes(),
    )?;
    write_bytes_atomic(
        &config.network_root().join(HTML_INDEX),
        render_html(&manifest, &batches).as_bytes(),
    )?;
    remove_stale_public_file(config, STALE_PUBLIC_BLOCKS_INDEX)?;

    Ok(CatalogGeneration {
        artifact_count,
        batch_count: batches.len(),
    })
}

fn read_batch_entries(config: &CollectorConfig) -> anyhow::Result<Vec<PublicBatchEntry>> {
    if !config.batches_root().exists() {
        return Ok(Vec::new());
    }

    let mut archive_paths = Vec::new();
    for entry in fs::read_dir(config.batches_root()).with_context(|| {
        format!(
            "failed to read batch export directory {}",
            config.batches_root().display()
        )
    })? {
        let entry = entry.with_context(|| {
            format!(
                "failed to read entry in batch export directory {}",
                config.batches_root().display()
            )
        })?;
        let path = entry.path();
        if entry
            .file_type()
            .with_context(|| format!("failed to read file type for {}", path.display()))?
            .is_file()
            && is_batch_archive(&path)
        {
            archive_paths.push(path);
        }
    }
    archive_paths.sort();

    let mut batches = Vec::new();
    for archive_path in archive_paths {
        let manifest = read_batch_manifest(&archive_path)?;
        ensure!(
            manifest.network == config.network,
            "batch archive {} is for network {}, expected {}",
            archive_path.display(),
            manifest.network,
            config.network
        );
        let byte_length = fs::metadata(&archive_path)
            .with_context(|| format!("failed to stat batch archive {}", archive_path.display()))?
            .len();
        let sha256 = artifact::file_sha256_hex(&archive_path)?;
        let relative_path = archive_path
            .strip_prefix(config.network_root())
            .with_context(|| {
                format!(
                    "batch archive {} is not under {}",
                    archive_path.display(),
                    config.network_root().display()
                )
            })?;
        batches.push(PublicBatchEntry {
            schema_version: CATALOG_SCHEMA_VERSION,
            network: manifest.network,
            batch_start_block: manifest.batch_start_block,
            batch_end_block: manifest.batch_end_block,
            batch_size: manifest.batch_size,
            artifact_count: manifest.artifact_count,
            created_at: manifest.created_at,
            byte_length,
            sha256,
            path: path_to_slash_string(relative_path),
        });
    }

    batches.sort_by_key(|batch| (batch.batch_start_block, batch.batch_end_block));
    Ok(batches)
}

fn public_manifest(
    config: &CollectorConfig,
    batches: &[PublicBatchEntry],
) -> anyhow::Result<PublicManifest> {
    let first_start_block = batches.iter().map(|entry| entry.batch_start_block).min();
    let last_end_block = batches.iter().map(|entry| entry.batch_end_block).max();
    let artifact_count = batches.iter().map(|entry| entry.artifact_count).sum();
    let total_byte_length = batches.iter().map(|entry| entry.byte_length).sum();

    Ok(PublicManifest {
        schema_version: CATALOG_SCHEMA_VERSION,
        kind: CATALOG_KIND.to_owned(),
        network: config.network.clone(),
        generated_at: artifact::utc_now_rfc3339()?,
        batch_size: config.batch_size,
        paths: PublicManifestPaths {
            html: HTML_INDEX.to_owned(),
            manifest: PUBLIC_MANIFEST.to_owned(),
            batches: PUBLIC_BATCHES_INDEX.to_owned(),
            checksums: CHECKSUMS.to_owned(),
            batch_prefix: BATCH_PREFIX.to_owned(),
        },
        batches: PublicBatchesSummary {
            count: batches.len(),
            artifact_count,
            first_start_block,
            last_end_block,
            total_byte_length,
        },
        notes: vec![
            "Public downloads are batch archives; individual block artifacts are inside those archives and are not published as standalone R2 objects.".to_owned(),
            "Cloudflare R2 public buckets do not provide directory listing; use this page or the JSON indexes instead.".to_owned(),
        ],
    })
}

fn remove_stale_public_file(config: &CollectorConfig, name: &str) -> anyhow::Result<()> {
    let path = config.network_root().join(name);
    if path.exists() {
        fs::remove_file(&path).with_context(|| {
            format!(
                "failed to remove stale public catalog file {}",
                path.display()
            )
        })?;
    }
    Ok(())
}

fn read_batch_manifest(path: &Path) -> anyhow::Result<BatchArchiveManifest> {
    let file = fs::File::open(path)
        .with_context(|| format!("failed to open batch archive {}", path.display()))?;
    let decoder = zstd::stream::read::Decoder::new(file)
        .with_context(|| format!("failed to create zstd decoder for {}", path.display()))?;
    let mut archive = Archive::new(decoder);
    for entry in archive
        .entries()
        .with_context(|| format!("failed to read tar entries from {}", path.display()))?
    {
        let mut entry =
            entry.with_context(|| format!("failed to read tar entry from {}", path.display()))?;
        if entry
            .path()
            .with_context(|| format!("failed to read tar entry path from {}", path.display()))?
            .as_ref()
            == Path::new("manifest.json")
        {
            let mut bytes = Vec::new();
            entry
                .read_to_end(&mut bytes)
                .with_context(|| format!("failed to read manifest.json from {}", path.display()))?;
            let manifest = serde_json::from_slice(&bytes).with_context(|| {
                format!("failed to decode manifest.json from {}", path.display())
            })?;
            return Ok(manifest);
        }
    }
    anyhow::bail!(
        "batch archive {} does not contain manifest.json",
        path.display()
    )
}

fn write_json<T>(path: PathBuf, value: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let bytes = serde_json::to_vec_pretty(value).context("failed to serialize JSON")?;
    write_bytes_atomic(&path, &bytes)
}

fn write_jsonl<T>(path: PathBuf, entries: &[T]) -> anyhow::Result<()>
where
    T: Serialize,
{
    let mut bytes = Vec::new();
    for entry in entries {
        serde_json::to_writer(&mut bytes, entry).context("failed to serialize JSONL entry")?;
        bytes.push(b'\n');
    }
    write_bytes_atomic(&path, &bytes)
}

fn checksums_file(batches: &[PublicBatchEntry]) -> String {
    let mut checksums = String::new();
    for batch in batches {
        checksums.push_str(batch.sha256.strip_prefix("0x").unwrap_or(&batch.sha256));
        checksums.push_str("  ");
        checksums.push_str(batch.path.rsplit('/').next().unwrap_or(&batch.path));
        checksums.push('\n');
    }
    checksums
}

fn render_html(manifest: &PublicManifest, batches: &[PublicBatchEntry]) -> String {
    let mut html = String::new();
    html.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n");
    html.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("<title>");
    push_escaped(&mut html, &manifest.network);
    html.push_str(" stateless inputs</title>\n<style>\n");
    html.push_str("body{font-family:system-ui,-apple-system,Segoe UI,sans-serif;line-height:1.5;margin:0;color:#1f2933;background:#f7f9fb}main{max-width:1080px;margin:0 auto;padding:32px 20px 48px}h1{font-size:32px;margin:0 0 8px}h2{font-size:20px;margin-top:32px}.summary{display:grid;grid-template-columns:repeat(auto-fit,minmax(160px,1fr));gap:12px;margin:24px 0}.metric{background:#fff;border:1px solid #d9e2ec;border-radius:8px;padding:14px}.metric strong{display:block;font-size:24px}.panel{background:#fff;border:1px solid #d9e2ec;border-radius:8px;padding:18px;margin:18px 0}code,pre{font-family:ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}pre{overflow:auto;background:#102a43;color:#f0f4f8;border-radius:8px;padding:14px}table{width:100%;border-collapse:collapse;background:#fff;border:1px solid #d9e2ec}th,td{text-align:left;border-bottom:1px solid #d9e2ec;padding:10px}th{background:#eef2f7}a{color:#0967d2}.muted{color:#627d98}.nowrap{white-space:nowrap}\n");
    html.push_str("</style>\n</head>\n<body>\n<main>\n");
    html.push_str("<h1>");
    push_escaped(&mut html, &manifest.network);
    html.push_str(" stateless inputs</h1>\n");
    html.push_str("<p class=\"muted\">Batch-first public dataset catalog generated at ");
    push_escaped(&mut html, &manifest.generated_at);
    html.push_str(".</p>\n");

    html.push_str("<section class=\"summary\">\n");
    push_metric(
        &mut html,
        "Block artifacts",
        manifest.batches.artifact_count,
    );
    push_metric(&mut html, "Batches", manifest.batches.count);
    push_metric(&mut html, "Batch size", manifest.batch_size);
    push_metric(
        &mut html,
        "Total batch bytes",
        manifest.batches.total_byte_length,
    );
    html.push_str("</section>\n");

    html.push_str("<section class=\"panel\">\n<h2>How to download</h2>\n");
    html.push_str("<p>Use the batch archive links below. Each archive contains block artifacts plus a <code>manifest.json</code>.</p>\n");
    if let Some(first_batch) = batches.first() {
        html.push_str("<pre>curl -LO ");
        push_escaped(&mut html, &first_batch.path);
        html.push_str("\ntar --zstd -xf ");
        push_escaped(
            &mut html,
            first_batch
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&first_batch.path),
        );
        html.push_str("</pre>\n");
    } else {
        html.push_str("<p>No complete batch archives are available yet.</p>\n");
    }
    html.push_str("<p>Verify downloads with <a href=\"");
    push_escaped_attr(&mut html, &manifest.paths.checksums);
    html.push_str("\"><code>SHA256SUMS</code></a>.</p>\n</section>\n");

    html.push_str("<section class=\"panel\">\n<h2>Machine-readable indexes</h2>\n<ul>\n");
    push_link_item(&mut html, &manifest.paths.manifest, "Dataset manifest");
    push_link_item(&mut html, &manifest.paths.batches, "Batch index");
    html.push_str("</ul>\n<p class=\"muted\">R2 public buckets do not provide directory listing; use these files instead of folder URLs.</p>\n</section>\n");

    html.push_str("<h2>Batch archives</h2>\n");
    if batches.is_empty() {
        html.push_str("<p>No completed batch archives have been exported yet.</p>\n");
    } else {
        html.push_str("<table>\n<thead><tr><th>Blocks</th><th>Artifacts</th><th>Size</th><th>SHA-256</th><th>Download</th></tr></thead>\n<tbody>\n");
        for batch in batches {
            html.push_str("<tr><td class=\"nowrap\">");
            push_escaped(
                &mut html,
                &format!("{}-{}", batch.batch_start_block, batch.batch_end_block),
            );
            html.push_str("</td><td>");
            push_escaped(&mut html, &batch.artifact_count.to_string());
            html.push_str("</td><td>");
            push_escaped(&mut html, &batch.byte_length.to_string());
            html.push_str("</td><td><code>");
            push_escaped(&mut html, &short_sha256(&batch.sha256));
            html.push_str("</code></td><td><a href=\"");
            push_escaped_attr(&mut html, &batch.path);
            html.push_str("\">");
            push_escaped(
                &mut html,
                batch.path.rsplit('/').next().unwrap_or(&batch.path),
            );
            html.push_str("</a></td></tr>\n");
        }
        html.push_str("</tbody>\n</table>\n");
    }

    html.push_str("</main>\n</body>\n</html>\n");
    html
}

fn push_metric<T>(html: &mut String, label: &str, value: T)
where
    T: std::fmt::Display,
{
    html.push_str("<div class=\"metric\"><span>");
    push_escaped(html, label);
    html.push_str("</span><strong>");
    push_escaped(html, &value.to_string());
    html.push_str("</strong></div>\n");
}

fn push_link_item(html: &mut String, href: &str, label: &str) {
    html.push_str("<li><a href=\"");
    push_escaped_attr(html, href);
    html.push_str("\"><code>");
    push_escaped(html, href);
    html.push_str("</code></a> ");
    push_escaped(html, label);
    html.push_str("</li>\n");
}

fn push_escaped_attr(out: &mut String, input: &str) {
    push_escaped(out, input);
}

fn push_escaped(out: &mut String, input: &str) {
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
}

fn short_sha256(sha256: &str) -> String {
    let hash = sha256.strip_prefix("0x").unwrap_or(sha256);
    match hash.get(..16) {
        Some(prefix) => format!("0x{prefix}..."),
        None => sha256.to_owned(),
    }
}

fn is_batch_archive(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".tar.zst"))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use alloy_primitives::B256;
    use serde_json::Value;
    use witness_generator_spec_cli::GeneratedInput;

    use crate::{
        artifact::{StatelessInputArtifact, append_index_entry, write_artifact_atomic},
        export,
    };

    use super::*;

    #[test]
    fn generates_public_catalog_for_completed_batches() {
        let config = test_config("completed_batches", 2);
        write_generated_artifact(&config, 0, B256::repeat_byte(0xaa));
        write_generated_artifact(&config, 1, B256::repeat_byte(0xbb));
        write_generated_artifact(&config, 2, B256::repeat_byte(0xcc));
        export::export_batches(&config, false).unwrap();

        let generation = generate_catalog(&config).unwrap();

        assert_eq!(generation.artifact_count, 2);
        assert_eq!(generation.batch_count, 1);
        assert!(config.network_root().join("index.html").is_file());
        assert!(config.network_root().join("manifest.json").is_file());
        assert!(config.network_root().join("batches.jsonl").is_file());
        assert!(!config.network_root().join("blocks.jsonl").exists());
        assert!(config.network_root().join("SHA256SUMS").is_file());

        let manifest: Value =
            serde_json::from_slice(&fs::read(config.network_root().join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(manifest["network"], "glamsterdam-devnet-5");
        assert!(manifest.get("blocks").is_none());
        assert!(manifest["paths"].get("blocks").is_none());
        assert!(manifest["paths"].get("legacyBlockIndex").is_none());
        assert_eq!(manifest["batches"]["count"], 1);
        assert_eq!(manifest["batches"]["artifactCount"], 2);

        let batches = read_jsonl_values(&config.network_root().join("batches.jsonl"));
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[0]["path"], "exports/batches/0-1.tar.zst");
        assert_eq!(
            batches[0]["byteLength"],
            fs::metadata(config.batches_root().join("0-1.tar.zst"))
                .unwrap()
                .len()
        );
        assert_eq!(
            batches[0]["sha256"],
            artifact::file_sha256_hex(&config.batches_root().join("0-1.tar.zst")).unwrap()
        );

        let checksums = fs::read_to_string(config.network_root().join("SHA256SUMS")).unwrap();
        assert!(checksums.contains("  0-1.tar.zst\n"));

        let html = fs::read_to_string(config.network_root().join("index.html")).unwrap();
        assert!(html.contains("glamsterdam-devnet-5 stateless inputs"));
        assert!(html.contains("exports/batches/0-1.tar.zst"));
        assert!(!html.contains("blocks.jsonl"));
        assert!(!html.contains("index.jsonl"));
    }

    #[test]
    fn incomplete_ranges_are_omitted_from_public_catalog() {
        let config = test_config("incomplete_ranges", 2);
        write_generated_artifact(&config, 3, B256::repeat_byte(0xdd));
        fs::write(config.network_root().join("blocks.jsonl"), b"stale\n").unwrap();
        export::export_batches(&config, false).unwrap();

        let generation = generate_catalog(&config).unwrap();

        assert_eq!(generation.artifact_count, 0);
        assert_eq!(generation.batch_count, 0);
        assert!(read_jsonl_values(&config.network_root().join("batches.jsonl")).is_empty());
        assert!(!config.network_root().join("blocks.jsonl").exists());
    }

    #[test]
    fn catalog_includes_existing_archives_when_export_skips_them() {
        let config = test_config("skipped_archives", 2);
        write_generated_artifact(&config, 0, B256::repeat_byte(0xaa));
        write_generated_artifact(&config, 1, B256::repeat_byte(0xbb));

        assert_eq!(export::export_batches(&config, false).unwrap().len(), 1);
        assert!(export::export_batches(&config, false).unwrap().is_empty());
        let generation = generate_catalog(&config).unwrap();

        assert_eq!(generation.artifact_count, 2);
        assert_eq!(generation.batch_count, 1);
        assert_eq!(
            read_jsonl_values(&config.network_root().join("batches.jsonl")).len(),
            1
        );
    }

    fn write_generated_artifact(config: &CollectorConfig, block_number: u64, block_hash: B256) {
        let generated = GeneratedInput {
            bytes: vec![0x15, 0x01, block_number as u8],
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
        let write = write_artifact_atomic(&config.blocks_root(), &artifact).unwrap();
        let index_entry = artifact.index_entry(&PathBuf::from("blocks").join(write.relative_path));
        append_index_entry(&config.index_path(), &index_entry).unwrap();
    }

    fn read_jsonl_values(path: &Path) -> Vec<Value> {
        let contents = fs::read_to_string(path).unwrap();
        contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    fn test_config(name: &str, batch_size: u64) -> CollectorConfig {
        let out_root = std::env::temp_dir().join(format!(
            "witness-generator-spec-cli-catalog-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&out_root);
        CollectorConfig {
            network: "glamsterdam-devnet-5".to_owned(),
            cl_url: "http://cl".to_owned(),
            el_url: "http://el".to_owned(),
            out_root,
            poll_interval: Duration::from_secs(4),
            request_timeout: Duration::from_secs(30),
            batch_size,
            r2: None,
        }
    }
}
