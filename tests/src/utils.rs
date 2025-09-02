//! Test utilities for the benchmark integration tests

#![cfg(test)]

use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use benchmark_runner::{
    guest_programs::{GuestIO, GuestMetadata, OutputVerifier},
    runner::{get_zkvm_instances, run_benchmark, Action, RunConfig},
    stateless_validator::ExecutionClient,
};
use ere_dockerized::ErezkVM;
use flate2::bufread::GzDecoder;
use tar::Archive;
use walkdir::WalkDir;
use zkevm_metrics::{BenchmarkRun, ExecutionMetrics, ProvingMetrics};
use zkvm_interface::ProverResourceType;

pub(crate) fn run_guest<T, OV>(
    guest_rel: &str,
    zkvms: &[ErezkVM],
    inputs: Vec<GuestIO<T, OV>>,
    output_folder: &Path,
    action: Action,
) where
    T: GuestMetadata,
    OV: OutputVerifier,
{
    let config = RunConfig {
        output_folder: output_folder.to_path_buf(),
        action,
        force_rerun: true,
    };
    let instances = get_zkvm_instances(
        zkvms,
        &PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("ere-guests"),
        Path::new(guest_rel),
        ProverResourceType::Cpu,
        true,
    )
    .unwrap();
    for zkvm in instances {
        run_benchmark(&zkvm, &config, inputs.clone()).unwrap();
    }

    assert!(
        std::fs::exists(output_folder.join("hardware.json")).unwrap(),
        "hardware.json file must exist"
    );
}

pub(crate) fn assert_executions_crashed<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: GuestMetadata,
{
    assert_execution_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ExecutionMetrics::Crashed { .. })
    });
}

pub(crate) fn assert_executions_successful<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: GuestMetadata,
{
    assert_execution_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ExecutionMetrics::Success { .. })
    });
}

fn assert_execution_status<F, Metadata>(
    output_path: &Path,
    expected_file_count: usize,
    predicate: F,
) where
    F: Fn(&ExecutionMetrics) -> bool,
    Metadata: GuestMetadata,
{
    let paths = get_result_files(output_path);
    assert_eq!(
        paths.len(),
        expected_file_count,
        "Expected {} result files, found {}",
        expected_file_count,
        paths.len()
    );
    for path in &paths {
        let result = BenchmarkRun::<Metadata>::from_path(path).unwrap();
        assert!(
            predicate(&result.execution.unwrap()),
            "Unexpected execution status for: {}",
            path.display()
        );
    }
}

pub(crate) fn assert_proving_successful<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: GuestMetadata,
{
    assert_proving_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ProvingMetrics::Success { .. })
    });
}

pub(crate) fn assert_proving_crashed<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: GuestMetadata,
{
    assert_proving_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ProvingMetrics::Crashed { .. })
    });
}

fn assert_proving_status<F, Metadata>(output_path: &Path, expected_file_count: usize, predicate: F)
where
    F: Fn(&ProvingMetrics) -> bool,
    Metadata: GuestMetadata,
{
    let paths = get_result_files(output_path);
    assert_eq!(
        paths.len(),
        expected_file_count,
        "Expected {} result files, found {}",
        expected_file_count,
        paths.len()
    );
    for path in &paths {
        let result = BenchmarkRun::<Metadata>::from_path(path).unwrap();
        assert!(
            predicate(&result.proving.unwrap()),
            "Unexpected proving status for: {}",
            path.display()
        );
    }
}

fn get_result_files(output_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(output_path)
        .min_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<_>>()
}

pub(crate) fn untar(path: &Path, dest_dir: &Path) {
    let file = File::open(path).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    let tar = GzDecoder::new(buf_reader);
    let mut archive = Archive::new(tar);
    archive.unpack(dest_dir).unwrap();
}

pub(crate) fn get_env_zkvm_or_default(default: Vec<ErezkVM>) -> Vec<ErezkVM> {
    std::env::var("ZKVM")
        .map(|zkvm| vec![ErezkVM::from_str(&zkvm).expect("Invalid ZKVM")])
        .unwrap_or(default)
}

pub(crate) fn filter_el_zkvm_pairs_from_env(
    default: Vec<(ExecutionClient, ErezkVM)>,
) -> Vec<(ExecutionClient, ErezkVM)> {
    let env_el = std::env::var("EL").ok().map(|el| {
        el.parse::<ExecutionClient>()
            .expect("Invalid execution client")
    });
    let env_zkvm = std::env::var("ZKVM")
        .ok()
        .map(|zkvm| zkvm.parse().expect("Invalid ZKVM"));
    let pairs = default
        .into_iter()
        .filter(|(el, zkvm)| {
            env_el.as_ref().is_none_or(|ref_el| el == ref_el)
                && env_zkvm.as_ref().is_none_or(|ref_zkvm| zkvm == ref_zkvm)
        })
        .collect::<Vec<_>>();
    assert!(!pairs.is_empty(), "No valid (EL, ZKVM) pairs found");
    pairs
}
