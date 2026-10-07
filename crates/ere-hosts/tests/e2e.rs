//! End-to-end runs of the `ere-hosts` binary with real guests on SP1.
//!
//! Each test executes and cost-estimates the real EEST fixtures in `testdata/eest` and requires
//! every result to match the fixture's expected output. The tests need Docker and network
//! access: Ere pulls `ere-server-sp1` from `ERE_IMAGE_REGISTRY` (default `ghcr.io/eth-act/ere`),
//! and the runner downloads the pinned guests from the ere-guests release. Set `GITHUB_TOKEN`
//! to avoid GitHub API rate limits.
//!
//! Ere names the server container after the zkVM, so these tests replace any running
//! `ere-server-sp1` container.
//!
//! Run them with `cargo test -p ere-hosts --test e2e -- --ignored`.

#![allow(
    unused_crate_dependencies,
    reason = "integration tests use only part of the package dependencies"
)]

use benchmark_runner::stateless_validator::{ExecutionClient, stateless_validator_input_iter};
use ere_dockerized::zkVMKind;
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
    process::Command,
    sync::{Mutex, PoisonError},
    thread,
    time::{Duration, Instant},
};

const DEFAULT_IMAGE_REGISTRY: &str = "ghcr.io/eth-act/ere";

/// Covers the image pull and SP1 prover startup, which dominate each run.
const RUN_TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// Ere runs a single `ere-server-sp1` container on a fixed port, so runs must not overlap.
static SP1_SERVER: Mutex<()> = Mutex::new(());

#[test]
#[ignore = "requires Docker and network access"]
fn reth_sp1_outputs_match_eest() {
    assert_outputs_match_eest(ExecutionClient::Reth);
}

#[test]
#[ignore = "requires Docker and network access"]
fn ethrex_sp1_outputs_match_eest() {
    assert_outputs_match_eest(ExecutionClient::Ethrex);
}

fn assert_outputs_match_eest(client: ExecutionClient) {
    let _guard = SP1_SERVER.lock().unwrap_or_else(PoisonError::into_inner);
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testdata/eest");
    let output = tempfile::tempdir().unwrap();

    // Both actions write to the same folder, so this also covers merging results per fixture.
    for action in ["execute", "estimate-cost"] {
        run_ere_hosts(client, action, &fixtures, output.path());
    }
    assert!(output.path().join("hardware.json").is_file());

    let expected_names: BTreeSet<String> =
        stateless_validator_input_iter(&fixtures, None, client, None)
            .unwrap()
            .map(|fixture| fixture.unwrap().name())
            .collect();
    let results_dir = output
        .path()
        .join(format!(
            "{}-{}",
            client_name(client),
            client.version().unwrap()
        ))
        .join(format!(
            "{}-{}",
            zkVMKind::SP1.name(),
            zkVMKind::SP1.sdk_version()
        ));
    let results = read_results(&results_dir);
    assert_eq!(
        results.keys().cloned().collect::<BTreeSet<_>>(),
        expected_names
    );

    for (name, run) in &results {
        for action in ["execution", "cost_estimation"] {
            assert_eq!(
                run[action]["success"]["output_matched"], true,
                "{client:?} {action} of {name} did not match the fixture output:\n{run:#}"
            );
        }
    }
}

fn run_ere_hosts(client: ExecutionClient, action: &str, input: &Path, output: &Path) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ere-hosts"));
    command
        .args(["--zkvms", "sp1", "--action", action, "--output-folder"])
        .arg(output)
        .args([
            "stateless-validator",
            "--execution-client",
            &client_name(client),
            "--input-folder",
        ])
        .arg(input);
    if env::var_os("ERE_IMAGE_REGISTRY").is_none() {
        command.env("ERE_IMAGE_REGISTRY", DEFAULT_IMAGE_REGISTRY);
    }

    let mut child = command.spawn().expect("failed to spawn ere-hosts");
    let deadline = Instant::now() + RUN_TIMEOUT;
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            panic!("ere-hosts {action} for {client:?} did not finish within {RUN_TIMEOUT:?}");
        }
        thread::sleep(Duration::from_secs(1));
    };
    assert!(
        status.success(),
        "ere-hosts {action} for {client:?} exited with {status}"
    );
}

/// Reads every `<fixture>.json` result in `dir`, keyed by fixture name.
fn read_results(dir: &Path) -> BTreeMap<String, serde_json::Value> {
    fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read results in {}: {err}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .map(|path| {
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            let run = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            (name, run)
        })
        .collect()
}

fn client_name(client: ExecutionClient) -> String {
    client.as_ref().to_lowercase()
}
