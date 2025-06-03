use anyhow::Result;
use async_trait::async_trait;
use ef_tests::{
    Case,
    cases::blockchain_test::{BlockchainTestCase, run_case},
};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::{BlocksAndWitnesses, witness_generator::WitnessGenerator};
use reth_stateless::ClientInput;

/// Root directory for the relevant blockchain tests within the `zkevm-fixtures` submodule.
const BLOCKCHAIN_TEST_DIR: &str = "blockchain_tests";

/// Witness generator that produces `BlocksAndWitnesses` for execution-spec-test fixtures.
#[derive(Debug, Clone)]
pub struct ExecSpecTestBlocksAndWitnesses {
    directory_path: PathBuf,
}
impl ExecSpecTestBlocksAndWitnesses {
    /// Creates a new instance of `ExecSpecTestBlocksAndWitnesses`.
    ///
    /// # Arguments
    ///
    /// * `directory_path` - The path to the directory containing the blockchain test cases.
    pub fn new(directory_path: PathBuf) -> Self {
        Self { directory_path }
    }
}

impl Default for ExecSpecTestBlocksAndWitnesses {
    fn default() -> Self {
        Self::new(path_to_zkevm_fixtures(BLOCKCHAIN_TEST_DIR))
    }
}

#[async_trait]
impl WitnessGenerator for ExecSpecTestBlocksAndWitnesses {
    /// Generates `BlocksAndWitnesses` for all valid blockchain test cases found
    /// within the specified `BLOCKCHAIN_TEST_DIR` directory in `zkevm-fixtures`.
    ///
    /// It walks the target directory, parses each JSON test file, executes the test
    /// using `ef_tests`, collects the resulting block/witness pairs, and packages them.
    ///
    /// Uses `rayon` for parallel processing of test cases within a single file.
    ///
    /// # Panics
    ///
    /// - If the `zkevm-fixtures` directory cannot be located relative to the crate root.
    /// - If the target `BLOCKCHAIN_TEST_DIR` directory does not exist.
    /// - If a JSON test case file cannot be parsed.
    /// - If `ef_tests::cases::blockchain_test::run_case` fails for a test.
    async fn generate(&self) -> Result<Vec<BlocksAndWitnesses>> {
        let suite_path = &self.directory_path;
        // Verify that the path exists
        assert!(
            suite_path.exists(),
            "Test suite path does not exist: {suite_path:?}"
        );

        // Find all files with the ".json" extension in the test suite directory
        // Each Json file corresponds to a BlockchainTestCase
        let test_cases: Vec<_> = find_all_files_with_extension(&suite_path, ".json")
            .into_iter()
            .map(|test_case_path| {
                let case =
                    BlockchainTestCase::load(&test_case_path).expect("test case should load");
                (test_case_path, case)
            })
            .collect();

        let mut blocks_and_witnesses = Vec::new();
        for (_, test_case) in test_cases {
            let blockchain_case: Vec<BlocksAndWitnesses> = test_case
                // Inside of a JSON file, we can have multiple tests, for example testopcode_Cancun,
                // testopcode_Prague
                // This is why we have `tests`.
                .tests
                .iter()
                .map(|(name, case)| BlocksAndWitnesses {
                    name: name.to_string(),
                    blocks_and_witnesses: run_case(case)
                        .unwrap()
                        .into_iter()
                        .map(|(recovered_block, witness)| ClientInput {
                            block: recovered_block.into_block(),
                            witness,
                        })
                        .collect(),
                    network: reth_stateless::fork_spec::ForkSpec::from(case.network),
                })
                .collect();
            blocks_and_witnesses.extend(blockchain_case);
        }

        Ok(blocks_and_witnesses)
    }
}

/// Recursively finds all files within `path` that end with `extension`.
// This function was copied from `ef-tests`
fn find_all_files_with_extension(path: &Path, extension: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(extension))
        .map(DirEntry::into_path)
        .collect()
}

/// Constructs the absolute path to a subdirectory within the `zkevm-fixtures` submodule.
///
/// Assumes this crate (`witness-generator`) is located at `<workspace-root>/crates/witness-generator`.
///
fn path_to_zkevm_fixtures(suite: &str) -> PathBuf {
    let workspace_root = Path::new(env!("CARGO_WORKSPACE_DIR"));
    workspace_root
        .join("zkevm-fixtures")
        .join("fixtures")
        .join(suite)
}
