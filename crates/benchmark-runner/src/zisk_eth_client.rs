//! Prebuilt-input path for the external `zisk-eth-client` stateless-validator
//! guest.
//!
//! The guest reads a single `RethInput` record and commits the validated block
//! hash. Its stdin cannot be produced by the ere-guests codec, so this module
//! converts the standard benchmark fixtures (the same
//! `StatelessValidationFixture` JSON files the workload emits) into the guest's
//! input in process via [`reth_input::convert_fixture_json`], then feeds them
//! through the normal execute/prove pipeline.

mod reth_input;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use ere_dockerized::Input;
use serde::Serialize;

use crate::guest_programs::{GenericGuestFixture, GuestFixture};

/// Metadata recorded alongside each converted `zisk-eth-client` fixture.
#[derive(Debug, Clone, Serialize)]
struct ZiskEthClientMetadata {
    /// Discriminates the fixture origin in the emitted metrics.
    fixture_format: &'static str,
    /// Fixture name.
    name: String,
}

/// Builds a lazy iterator of guest fixtures from a folder of benchmark fixtures.
///
/// Each `*.json` file is parsed and converted into the guest's stdin and
/// expected committed output. When `fixture_filter` is set, only files whose
/// stem contains one of the provided substrings are emitted.
pub fn zisk_eth_client_input_iter(
    input_folder: &Path,
    fixture_filter: Option<&[String]>,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>> + Send> {
    let mut paths: Vec<PathBuf> = fs::read_dir(input_folder)
        .with_context(|| format!("Failed to read input folder: {}", input_folder.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();

    let filter: Option<Vec<String>> = fixture_filter.map(<[String]>::to_vec);

    Ok(paths
        .into_iter()
        .filter(move |path| match &filter {
            Some(needles) => {
                let stem = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("");
                needles.iter().any(|needle| stem.contains(needle.as_str()))
            }
            None => true,
        })
        .map(|path| {
            let bytes = fs::read(&path)
                .with_context(|| format!("Failed to read fixture {}", path.display()))?;
            let (name, stdin, expected) = reth_input::convert_fixture_json(&bytes)
                .with_context(|| format!("Failed to convert fixture {}", path.display()))?;
            let metadata = ZiskEthClientMetadata {
                fixture_format: "zisk-eth-client-reth",
                name: name.clone(),
            };
            let fixture = GenericGuestFixture {
                name,
                input: Input::new().with_stdin(stdin),
                expected_public_values: expected,
                metadata,
            };
            Ok(Box::new(fixture) as Box<dyn GuestFixture>)
        }))
}
