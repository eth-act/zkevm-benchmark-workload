use ef_tests::{
    Case,
    cases::blockchain_test::{BlockchainTestCase, run_case},
};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use crate::BlocksAndWitnesses;
use reth_stateless::ClientInput;

/// Root directory for the relevant blockchain tests within the `zkevm-fixtures` submodule.
const BLOCKCHAIN_TEST_DIR: &str = "blockchain_tests";

/// Load pre-generated witnesses from the `zkevm-fixtures-with-witnesses` directory.
/// 
/// If the directory doesn't exist or witnesses can't be loaded, returns an error
/// with instructions for the user to generate witnesses first.
/// 
/// # Errors
/// 
/// Returns an error if:
/// - The witness directory doesn't exist
/// - Witness files can't be read or parsed
/// - No witnesses are found
pub fn load_pre_generated_witnesses() -> Result<Vec<BlocksAndWitnesses>, Box<dyn std::error::Error>> {
    let workspace_root = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let witness_dir = workspace_root.join("zkevm-fixtures-with-witnesses");
    
    if !witness_dir.exists() {
        return Err(format!(
            "Pre-generated witness directory not found: {}\n\
            Please generate witnesses first by running:\n\
            cargo run --bin witness-generator",
            witness_dir.display()
        ).into());
    }
    
    let index_path = witness_dir.join("index.json");
    if !index_path.exists() {
        return Err(format!(
            "Witness index file not found: {}\n\
            The witness directory exists but appears to be incomplete.\n\
            Please regenerate witnesses by running:\n\
            cargo run --bin witness-generator",
            index_path.display()
        ).into());
    }
    
    // Read the index to get list of available witnesses
    let index_content = std::fs::read_to_string(&index_path)?;
    let witness_names: Vec<String> = serde_json::from_str(&index_content)?;
    
    if witness_names.is_empty() {
        return Err("No witnesses found in index file. Please regenerate witnesses.".into());
    }
    
    println!("Loading {} pre-generated witnesses from {}", witness_names.len(), witness_dir.display());
    
    let mut witnesses = Vec::new();
    
    for name in &witness_names {
        let filename = format!("{}.json", sanitize_filename(name));
        let file_path = witness_dir.join(filename);
        
        if !file_path.exists() {
            return Err(format!(
                "Witness file not found: {}\n\
                The witness directory appears to be incomplete. Please regenerate witnesses.",
                file_path.display()
            ).into());
        }
        
        let content = std::fs::read_to_string(&file_path)?;
        let witness: BlocksAndWitnesses = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse witness file {}: {}", file_path.display(), e))?;
        
        witnesses.push(witness);
    }
    
    println!("Successfully loaded {} witnesses", witnesses.len());
    Ok(witnesses)
}

/// Sanitize a filename by replacing invalid characters with underscores
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '[' | ']' => '_',
            c => c,
        })
        .collect()
}

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
pub fn generate() -> Vec<BlocksAndWitnesses> {
    // First get the path to "BLOCKCHAIN_TEST_DIR"
    // TODO: Maybe we should have this be passed as a parameter in the future
    let suite_path = path_to_zkevm_fixtures(BLOCKCHAIN_TEST_DIR);
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
            let case = BlockchainTestCase::load(&test_case_path).expect("test case should load");
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

    blocks_and_witnesses
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
