//! Download and extract EEST fixtures from GitHub releases.

use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;
use tracing::info;

use crate::{Result, WGError};

/// GitHub repository for execution spec tests.
const REPO: &str = "ethereum/execution-spec-tests";

/// Asset filename to download from the release.
const ASSET_NAME: &str = "fixtures_benchmark.tar.gz";

/// Default release tag.
const DEFAULT_TAG: &str = "benchmark@v0.0.7";

/// Downloads and extracts EEST fixtures from a GitHub release.
///
/// # Arguments
/// * `tag` - Optional release tag. `None` uses the default tag (`benchmark@v0.0.7`).
///   `Some("latest")` resolves the latest non-prerelease semver release.
/// * `dest_dir` - Directory where fixtures will be extracted.
pub(crate) async fn download_and_extract(tag: Option<&str>, dest_dir: &Path) -> Result<()> {
    let client = build_http_client()?;
    let resolved_tag = resolve_tag(&client, tag).await?;
    let download_url = get_asset_download_url(&client, &resolved_tag).await?;
    download_and_untar(&client, &download_url, dest_dir).await
}

/// Builds an HTTP client with GitHub API headers and optional token auth.
fn build_http_client() -> Result<reqwest::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("zkevm-benchmark-workload"),
    );

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        info!("Using GitHub token for API authentication");
        let value = format!("Bearer {token}");
        headers.insert(
            reqwest::header::AUTHORIZATION,
            value
                .parse()
                .map_err(|_| WGError::DownloadFailed("invalid GITHUB_TOKEN value".to_string()))?,
        );
    }

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| WGError::DownloadFailed(format!("failed to build HTTP client: {e}")))
}

/// Resolves the tag to use for the GitHub release.
async fn resolve_tag(client: &reqwest::Client, tag: Option<&str>) -> Result<String> {
    match tag {
        None => {
            info!("Using default tag: {DEFAULT_TAG}");
            Ok(DEFAULT_TAG.to_string())
        }
        Some("latest") => {
            info!("Finding latest official release...");
            resolve_latest_tag(client).await
        }
        Some(t) => {
            info!("Using specified tag: {t}");
            Ok(t.to_string())
        }
    }
}

/// Queries the GitHub API to find the latest non-prerelease semver release.
async fn resolve_latest_tag(client: &reqwest::Client) -> Result<String> {
    let url = format!("https://api.github.com/repos/{REPO}/releases");
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("failed to list releases: {e}")))?;

    if !resp.status().is_success() {
        return Err(WGError::DownloadFailed(format!(
            "GitHub API returned {} when listing releases",
            resp.status()
        )));
    }

    let releases: Vec<serde_json::Value> = resp
        .json()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("failed to parse releases JSON: {e}")))?;

    for release in &releases {
        let prerelease = release["prerelease"].as_bool().unwrap_or(true);
        if prerelease {
            continue;
        }
        if let Some(tag_name) = release["tag_name"].as_str()
            && is_semver_tag(tag_name)
        {
            info!("Using latest official release: {tag_name}");
            return Ok(tag_name.to_string());
        }
    }

    Err(WGError::DownloadFailed(format!(
        "no official release found in {REPO}"
    )))
}

/// Queries the GitHub API to get the download URL for the fixture asset.
async fn get_asset_download_url(client: &reqwest::Client, tag: &str) -> Result<String> {
    let encoded_tag = tag.replace('@', "%40");
    let url = format!("https://api.github.com/repos/{REPO}/releases/tags/{encoded_tag}");

    info!("Getting release info for {tag}...");
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("failed to get release {tag}: {e}")))?;

    if !resp.status().is_success() {
        return Err(WGError::DownloadFailed(format!(
            "GitHub API returned {} for tag {tag}",
            resp.status()
        )));
    }

    let release: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("failed to parse release JSON: {e}")))?;

    let assets = release["assets"]
        .as_array()
        .ok_or_else(|| WGError::DownloadFailed("no assets in release".to_string()))?;

    for asset in assets {
        if asset["name"].as_str() == Some(ASSET_NAME)
            && let Some(url) = asset["browser_download_url"].as_str()
        {
            return Ok(url.to_string());
        }
    }

    Err(WGError::DownloadFailed(format!(
        "asset {ASSET_NAME} not found in release {tag}"
    )))
}

/// Downloads the fixture archive and extracts it to the destination directory.
async fn download_and_untar(client: &reqwest::Client, url: &str, dest_dir: &Path) -> Result<()> {
    info!("Downloading {ASSET_NAME}...");
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("download failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(WGError::DownloadFailed(format!(
            "download returned status {}",
            resp.status()
        )));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| WGError::DownloadFailed(format!("failed to read response body: {e}")))?;

    info!("Extracting {} bytes to {}", bytes.len(), dest_dir.display());

    let dest = dest_dir.to_path_buf();
    tokio::task::spawn_blocking(move || {
        std::fs::create_dir_all(&dest)?;
        let decoder = GzDecoder::new(&bytes[..]);
        let mut archive = Archive::new(decoder);
        archive.unpack(&dest)?;
        Ok::<(), std::io::Error>(())
    })
    .await
    .map_err(|e| WGError::DownloadFailed(format!("extraction task failed: {e}")))?
    .map_err(|e| WGError::DownloadFailed(format!("extraction failed: {e}")))?;

    info!("Fixtures ready in {}", dest_dir.display());
    Ok(())
}

/// Checks if a tag matches the semver pattern `v<major>.<minor>.<patch>`.
fn is_semver_tag(tag: &str) -> bool {
    let Some(rest) = tag.strip_prefix('v') else {
        return false;
    };
    let parts: Vec<&str> = rest.split('.').collect();
    parts.len() == 3 && parts.iter().all(|p| p.parse::<u64>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "slow-tests")]
    #[tokio::test]
    async fn test_download_and_extract_default_tag() {
        let dir = tempfile::tempdir().unwrap();
        download_and_extract(None, dir.path()).await.unwrap();
        assert!(
            dir.path().join("fixtures/blockchain_tests").exists(),
            "expected fixtures/blockchain_tests directory after extraction"
        );
    }
}
