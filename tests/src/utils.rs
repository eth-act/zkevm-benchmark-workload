//! Test utilities for the benchmark integration tests

#![cfg(test)]

use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

use alloy_primitives::{map::B256Map, B256};
use benchmark_runner::{
    guest_programs::GuestFixture,
    runner::{get_zkvm_instances, run_benchmark, Action, RunConfig},
    stateless_validator::ExecutionClient,
};
use ere_dockerized::zkVMKind;
use ere_platform_trait::Platform;
use ere_zkvm_interface::ProverResourceType;
use flate2::bufread::GzDecoder;
use reth_errors::ProviderError;
use reth_stateless::validation::StatelessValidationError;
use reth_trie_common::HashedPostState;
use revm_bytecode::Bytecode;
use serde::{de::DeserializeOwned, Serialize};
use tar::Archive;
use walkdir::WalkDir;
use zkevm_metrics::{BenchmarkRun, ExecutionMetrics, ProvingMetrics};

pub(crate) fn run_guest(
    guest_rel: &str,
    zkvms: &[zkVMKind],
    inputs: Vec<impl GuestFixture>,
    output_folder: &Path,
    sub_folder: Option<String>,
    action: Action,
) {
    let config = RunConfig {
        output_folder: output_folder.to_path_buf(),
        sub_folder,
        action,
        force_rerun: true,
        dump_inputs_folder: None,
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
        run_benchmark(&zkvm, &config, &inputs).unwrap();
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
    Metadata: Serialize + DeserializeOwned,
{
    assert_execution_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ExecutionMetrics::Crashed { .. })
    });
}

pub(crate) fn assert_executions_successful<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: Serialize + DeserializeOwned,
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
    Metadata: Serialize + DeserializeOwned,
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
            "Unexpected execution status for: {} (content={})",
            path.display(),
            std::fs::read_to_string(path).unwrap_or_default()
        );
    }
}

pub(crate) fn assert_proving_successful<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: Serialize + DeserializeOwned,
{
    assert_proving_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ProvingMetrics::Success { .. })
    });
}

pub(crate) fn assert_proving_crashed<Metadata>(
    metrics_folder_path: &Path,
    expected_file_count: usize,
) where
    Metadata: Serialize + DeserializeOwned,
{
    assert_proving_status::<_, Metadata>(metrics_folder_path, expected_file_count, |exec| {
        matches!(exec, ProvingMetrics::Crashed { .. })
    });
}

fn assert_proving_status<F, Metadata>(output_path: &Path, expected_file_count: usize, predicate: F)
where
    F: Fn(&ProvingMetrics) -> bool,
    Metadata: Serialize + DeserializeOwned,
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
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
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

pub(crate) fn get_env_zkvm_or_default(default: Vec<zkVMKind>) -> Vec<zkVMKind> {
    std::env::var("ZKVM")
        .map(|zkvm| vec![zkVMKind::from_str(&zkvm).expect("Invalid ZKVM")])
        .unwrap_or(default)
}

pub(crate) fn filter_el_zkvm_pairs_from_env(
    default: Vec<(ExecutionClient, zkVMKind)>,
) -> Vec<(ExecutionClient, zkVMKind)> {
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

#[derive(Debug)]
pub(crate) struct Wrapper<T: reth_stateless::StatelessTrie> {
    inner: T,
}

impl<T: reth_stateless::StatelessTrie> reth_stateless::StatelessTrie for Wrapper<T> {
    fn new(
        witness: &reth_stateless::ExecutionWitness,
        pre_state_root: B256,
    ) -> Result<(Self, B256Map<Bytecode>), StatelessValidationError>
    where
        Self: Sized,
    {
        println!(
            "StatelessTrie::new called with pre_state_root: {:?}",
            pre_state_root
        );
        // Note: We can't delegate `new` to `internal` since `internal` is already constructed.
        // This method would be used to create a new Wrapper from scratch.
        // For now, we'll use the default StatelessSparseTrie implementation.
        let (inner, bytecodes) = T::new(witness, pre_state_root)?;
        let wrapper = Self { inner };
        println!(
            "StatelessTrie::new returning with {} bytecodes",
            bytecodes.len()
        );
        Ok((wrapper, bytecodes))
    }

    fn account(
        &self,
        address: alloy_primitives::Address,
    ) -> Result<Option<reth_trie_common::TrieAccount>, ProviderError> {
        println!("StatelessTrie::account called with address: {:?}", address);
        let result = self.inner.account(address);
        println!("StatelessTrie::account returning: {:?}", result);
        result
    }

    fn storage(
        &self,
        address: alloy_primitives::Address,
        slot: alloy_primitives::U256,
    ) -> Result<alloy_primitives::U256, ProviderError> {
        println!(
            "StatelessTrie::storage called with address: {:?}, slot: {:?}",
            address, slot
        );
        let result = self.inner.storage(address, slot);
        println!("StatelessTrie::storage returning: {:?}", result);
        result
    }

    fn calculate_state_root(
        &mut self,
        state: HashedPostState,
    ) -> Result<B256, StatelessValidationError> {
        println!(
            "StatelessTrie::calculate_state_root called with {} accounts, {} storages",
            state.accounts.len(),
            state.storages.len()
        );
        let result = self.inner.calculate_state_root(state);
        println!(
            "StatelessTrie::calculate_state_root returning: {:?}",
            result
        );
        result
    }
}

pub(crate) struct NoopPlatform;

impl Platform for NoopPlatform {
    fn read_whole_input() -> impl std::ops::Deref<Target = [u8]> {
        panic!("NoopPlatform does not implement read_whole_input");
        #[allow(unreachable_code)]
        Vec::<u8>::new()
    }

    fn write_whole_output(output: &[u8]) {
        println!(
            "NoopPlatform: received output with length: {}",
            output.len()
        );
    }

    fn print(message: &str) {
        println!("NoopPlatform: message: {}", message);
    }
}
