//! Build script that extracts resolved dependency metadata from `Cargo.lock`
//! and exposes it as compile-time environment variables.
//!
//! It traces through the ere-guests dependency chain: finds the
//! `stateless-validator-reth` / `stateless-validator-ethrex` packages, looks at
//! their dependencies, and extracts the git tag (or short commit hash) from the
//! resolved source of the first matching EL dependency.

use std::env;
use std::fs;

/// Dependency prefix used to identify reth crates inside `stateless-validator-reth`.
const RETH_DEP_PREFIX: &str = "reth-";
/// Dependency prefix used to identify ethrex crates inside `stateless-validator-ethrex`.
const ETHREX_DEP_PREFIX: &str = "ethrex-";
/// Repository URL used by Cargo for ere-guests git dependencies.
const ERE_GUESTS_REPO: &str = "https://github.com/eth-act/ere-guests";
/// Package used to resolve the ere-guests source for guest artifact downloads.
const ERE_GUESTS_DOWNLOAD_PACKAGE_NAME: &str = "downloader";

enum GuestDownloadSource {
    Tag(String),
    Commit(String),
}

fn main() {
    let workspace_dir = env::var("CARGO_WORKSPACE_DIR").expect("CARGO_WORKSPACE_DIR not set");

    let lockfile_path = format!("{workspace_dir}/Cargo.lock");
    println!("cargo:rerun-if-changed={lockfile_path}");

    let lockfile_content = fs::read_to_string(&lockfile_path).expect("Failed to read Cargo.lock");

    let lockfile: toml::Value =
        toml::from_str(&lockfile_content).expect("Failed to parse Cargo.lock");

    let packages = lockfile
        .get("package")
        .and_then(|v| v.as_array())
        .expect("No package array in Cargo.lock");

    let reth_version = find_el_version(packages, "stateless-validator-reth", RETH_DEP_PREFIX);
    let ethrex_version = find_el_version(packages, "stateless-validator-ethrex", ETHREX_DEP_PREFIX);
    let guest_download_source = find_ere_guests_download_source(packages);

    println!(
        "cargo:rustc-env=RETH_EL_VERSION={}",
        reth_version.unwrap_or_else(|| {
            println!("cargo:warning=Could not determine reth version from Cargo.lock");
            "unknown".to_string()
        })
    );
    println!(
        "cargo:rustc-env=ETHREX_EL_VERSION={}",
        ethrex_version.unwrap_or_else(|| {
            println!("cargo:warning=Could not determine ethrex version from Cargo.lock");
            "unknown".to_string()
        })
    );

    match guest_download_source {
        Some(GuestDownloadSource::Tag(tag)) => {
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_KIND=tag");
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_VALUE={tag}");
        }
        Some(GuestDownloadSource::Commit(commit)) => {
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_KIND=commit");
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_VALUE={commit}");
        }
        None => {
            println!("cargo:warning=Could not determine ere-guests source from Cargo.lock");
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_KIND=unknown");
            println!("cargo:rustc-env=ERE_GUESTS_DOWNLOAD_VALUE=unknown");
        }
    }

    let (zilkworm_tag, zilkworm_sha, zilkworm_repo_api_url) = find_zilkworm_git_ref(packages);
    println!("cargo:rustc-env=ZILKWORM_GUEST_TAG={zilkworm_tag}");
    println!("cargo:rustc-env=ZILKWORM_GUEST_SHA={zilkworm_sha}");
    println!("cargo:rustc-env=ZILKWORM_GUEST_REPO_API_URL={zilkworm_repo_api_url}");

    let zilkworm_version = if !zilkworm_tag.is_empty() {
        zilkworm_tag
    } else if !zilkworm_sha.is_empty() {
        zilkworm_sha[..7.min(zilkworm_sha.len())].to_string()
    } else {
        "local".to_string()
    };
    println!("cargo:rustc-env=ZILKWORM_EL_VERSION={zilkworm_version}");
}

fn find_zilkworm_git_ref(packages: &[toml::Value]) -> (String, String, String) {
    let Some(pkg) = packages
        .iter()
        .find(|p| p.get("name").and_then(|n| n.as_str()) == Some("z6m_stateless_validator"))
    else {
        return (String::new(), String::new(), String::new());
    };
    let Some(source) = pkg.get("source").and_then(|s| s.as_str()) else {
        return (String::new(), String::new(), String::new());
    };
    let tag = extract_tag_from_source(source).unwrap_or_default();
    let sha = extract_hash_from_source(source)
        .unwrap_or_default()
        .to_string();
    let api_url = extract_repo_api_url_from_source(source).unwrap_or_default();
    (tag, sha, api_url)
}

fn extract_repo_api_url_from_source(source: &str) -> Option<String> {
    let url = source.strip_prefix("git+")?;
    let url = url.split(['?', '#']).next()?;
    let path = url.strip_prefix("https://github.com/")?;
    Some(format!("https://api.github.com/repos/{path}"))
}

/// Finds the EL version by tracing the ere-guests dependency chain:
/// 1. Locate `guest_pkg_name` (e.g. `stateless-validator-reth`) in the lockfile.
/// 2. Find the first dependency whose name starts with `dep_prefix` (e.g. `reth-`).
/// 3. Look up that dependency's `[[package]]` entry and extract the tag or short
///    commit hash from its `source` field.
fn find_el_version(
    packages: &[toml::Value],
    guest_pkg_name: &str,
    dep_prefix: &str,
) -> Option<String> {
    // Step 1: find the guest package and its dependency list.
    let guest_pkg = packages
        .iter()
        .find(|p| p.get("name").and_then(|n| n.as_str()) == Some(guest_pkg_name))?;

    let deps = guest_pkg.get("dependencies").and_then(|d| d.as_array())?;

    // Step 2: pick the first dependency that matches the EL prefix.
    let el_dep_name = deps
        .iter()
        .filter_map(|d| d.as_str())
        .find(|name| name.starts_with(dep_prefix))?;

    // Step 3: look up that dependency's source.
    let el_pkg = packages
        .iter()
        .find(|p| p.get("name").and_then(|n| n.as_str()) == Some(el_dep_name))?;

    let source = el_pkg.get("source").and_then(|s| s.as_str())?;

    extract_tag_from_source(source).or_else(|| extract_short_hash_from_source(source))
}

/// Extracts the git tag from a Cargo.lock source string.
///
/// Source format: `git+https://github.com/.../repo?tag=v1.10.2#8e3b5e6a...`
fn extract_tag_from_source(source: &str) -> Option<String> {
    extract_query_param_from_source(source, "tag")
}

/// Extracts the short (7-char) commit hash from a Cargo.lock source string.
///
/// Source format: `git+https://github.com/.../repo?rev=abc123#abc123def456...`
fn extract_short_hash_from_source(source: &str) -> Option<String> {
    extract_hash_from_source(source).map(|hash| hash[..7.min(hash.len())].to_string())
}

fn find_ere_guests_download_source(packages: &[toml::Value]) -> Option<GuestDownloadSource> {
    let source = packages.iter().find_map(|package| {
        let name = package.get("name").and_then(|n| n.as_str())?;
        if name != ERE_GUESTS_DOWNLOAD_PACKAGE_NAME {
            return None;
        }

        let source = package.get("source").and_then(|s| s.as_str())?;
        source.contains(ERE_GUESTS_REPO).then_some(source)
    })?;

    extract_tag_from_source(source)
        .map(GuestDownloadSource::Tag)
        .or_else(|| {
            extract_hash_from_source(source)
                .map(|hash| GuestDownloadSource::Commit(hash.to_string()))
        })
}

fn extract_query_param_from_source(source: &str, param: &str) -> Option<String> {
    let query = source.split('?').nth(1)?;
    let query = query.split('#').next().unwrap_or(query);
    let prefix = format!("{param}=");

    query.split('&').find_map(|part| {
        part.strip_prefix(&prefix)
            .map(std::string::ToString::to_string)
    })
}

fn extract_hash_from_source(source: &str) -> Option<&str> {
    let hash = source.split('#').nth(1)?;
    (!hash.is_empty()).then_some(hash)
}
