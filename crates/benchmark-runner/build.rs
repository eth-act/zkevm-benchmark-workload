//! Build script that extracts execution client (reth/ethrex) versions from
//! `Cargo.lock` and exposes them as compile-time environment variables.
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
    let ere_tag = find_ere_tag(packages);

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
    println!(
        "cargo:rustc-env=ERE_TAG={}",
        ere_tag.unwrap_or_else(|| {
            println!("cargo:warning=Could not determine ERE tag from Cargo.lock");
            "unknown".to_string()
        })
    );
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
    let after_tag = source.split("?tag=").nth(1)?;
    let tag_end = after_tag.find('#').unwrap_or(after_tag.len());
    Some(after_tag[..tag_end].to_string())
}

/// Extracts the short (7-char) commit hash from a Cargo.lock source string.
///
/// Source format: `git+https://github.com/.../repo?rev=abc123#abc123def456...`
fn extract_short_hash_from_source(source: &str) -> Option<String> {
    let hash = source.split('#').nth(1)?;
    Some(hash[..7.min(hash.len())].to_string())
}

/// Finds the ERE tag (short commit hash) from the `ere-platform-trait` package source.
fn find_ere_tag(packages: &[toml::Value]) -> Option<String> {
    let pkg = packages
        .iter()
        .find(|p| p.get("name").and_then(|n| n.as_str()) == Some("ere-platform-trait"))?;

    let source = pkg.get("source").and_then(|s| s.as_str())?;
    extract_short_hash_from_source(source)
}
