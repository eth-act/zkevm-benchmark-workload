use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use ef_tests::{
    Case,
    cases::blockchain_test::{BlockchainTestCase, run_case},
    models::BlockchainTest,
};
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::{DirEntry, WalkDir};

use crate::{BlocksAndWitnesses, blocks_and_witnesses::WitnessGenerator};
use reth_stateless::{StatelessInput, fork_spec::ForkSpec};

/// Witness generator that produces `BlocksAndWitnesses` for execution-spec-test fixtures.
#[derive(Debug, Clone, Default)]
pub struct ExecSpecTestBlocksAndWitnessBuilder {
    input_folder: Option<PathBuf>,
    tag: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl ExecSpecTestBlocksAndWitnessBuilder {
    /// Sets the tag for the execution-spec-test fixtures.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Sets the input folder for the execution-spec-test fixtures.
    /// Returns an error if the path doesn't exist or isn't a directory.
    pub fn with_input_folder(mut self, path: PathBuf) -> Result<Self> {
        if !path.exists() {
            bail!("EEST fixtures path '{}' does not exist", path.display());
        }
        if !path.is_dir() {
            bail!("EEST fixtures path '{}' is not a directory", path.display());
        }
        let canonical_path = path
            .canonicalize()
            .with_context(|| format!("Failed to resolve path '{}'", path.display()))?;

        self.input_folder = Some(canonical_path);
        Ok(self)
    }

    /// Includes only test names that contain the provided strings.
    pub fn with_includes(mut self, includes: Vec<String>) -> Self {
        self.include = Some(includes);
        self
    }

    /// Excludes all test names that contain the provided strings.
    pub fn with_excludes(mut self, exclude: Vec<String>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    /// Builds the `ExecSpecTestBlocksAndWitnesses` instance.
    pub fn build(self) -> Result<ExecSpecTestBlocksAndWitnesses> {
        let input_folder = self.input_folder;
        let tag = self.tag;
        let include = self.include.unwrap_or_default();
        let exclude = self.exclude.unwrap_or_default();

        let (directory_path, delete_eest_folder) = if let Some(input_folder) = input_folder {
            (input_folder, false)
        } else {
            let mut cmd = Command::new("./scripts/download-and-extract-fixtures.sh");
            if let Some(tag) = tag {
                cmd.arg(tag);
            }
            let output = cmd.output().context("Failed to execute download script")?;

            if !output.status.success() {
                bail!(
                    "Failed to download EEST benchmark fixtures: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            (PathBuf::from("./zkevm-fixtures"), true)
        };

        Ok(ExecSpecTestBlocksAndWitnesses {
            directory_path,
            include,
            exclude,
            delete_eest_folder,
        })
    }
}

/// Witness generator that produces `BlocksAndWitnesses` for EEST fixtures.
#[derive(Debug, Clone)]
pub struct ExecSpecTestBlocksAndWitnesses {
    directory_path: PathBuf,
    include: Vec<String>,
    exclude: Vec<String>,
    delete_eest_folder: bool,
}

impl Drop for ExecSpecTestBlocksAndWitnesses {
    fn drop(&mut self) {
        if self.delete_eest_folder && self.directory_path.exists() {
            match std::fs::remove_dir_all(&self.directory_path) {
                Ok(_) => {}
                Err(e) => eprintln!(
                    "Failed to remove directory {}: {}",
                    self.directory_path.display(),
                    e
                ),
            }
        }
    }
}

#[async_trait]
impl WitnessGenerator for ExecSpecTestBlocksAndWitnesses {
    // Generates blocks and witnesses from the EEST fixtures located in the specified directory,
    // filtering by the provided include and exclude patterns.
    async fn generate(&self) -> Result<Vec<BlocksAndWitnesses>> {
        let suite_path = self.directory_path.join("fixtures/blockchain_tests");

        if !suite_path.exists() {
            bail!("Test suite path does not exist: {}.", suite_path.display());
        }

        let test_file_paths = find_all_files_with_extension(&suite_path, ".json");
        let mut tests: Vec<(String, BlockchainTest)> = Vec::new();
        for path in test_file_paths {
            let test_case = match BlockchainTestCase::load(&path) {
                Ok(case) => case,
                Err(e) => {
                    eprintln!("Failed to load test case from {}: {e}", path.display());
                    continue;
                }
            };

            let file_tests: Vec<(String, BlockchainTest)> = test_case
                .tests
                .into_iter()
                .map(|(name, case)| {
                    (
                        name.split('/').next_back().unwrap_or(&name).to_string(),
                        case,
                    )
                })
                .filter(|(name, _)| !self.exclude.iter().any(|filter| name.contains(filter)))
                .filter(|(name, _)| self.include.iter().all(|f| name.contains(f)))
                .map(|(name, case)| (name, case))
                .collect();
            tests.extend(file_tests);
        }

        let bws: Result<Vec<_>> = tests
            .par_iter()
            .map(|(name, case)| {
                Ok(BlocksAndWitnesses {
                    name: name.to_string(),
                    block_and_witness: run_case(case)?
                        .into_iter()
                        .next_back()
                        .map(|(recovered_block, witness)| StatelessInput {
                            block: recovered_block.into_block(),
                            witness,
                        })
                        .ok_or_else(|| anyhow!("No target block found for test case {}", name))?,
                    network: ForkSpec::from(case.network),
                })
            })
            .collect();

        bws
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eest_generator_deletes_directory_on_drop() {
        for delete_eest_folder in [false, true] {
            let directory_path = PathBuf::from("./zkevm-fixtures/test-123");
            std::fs::create_dir_all(&directory_path).unwrap();
            assert!(directory_path.exists());

            let eest_generator = ExecSpecTestBlocksAndWitnesses {
                directory_path: directory_path.clone(),
                include: vec![],
                exclude: vec![],
                delete_eest_folder,
            };

            drop(eest_generator);

            assert_eq!(
                !directory_path.exists(),
                delete_eest_folder,
                "Directory should {}exist after drop",
                if delete_eest_folder { "" } else { "not " }
            );
        }
    }
}
