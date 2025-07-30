//! Generate fixtures for zkEVM benchmarking tool

use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use ef_tests::{
    Case,
    cases::blockchain_test::{BlockchainTestCase, run_case},
    models::BlockchainTest,
};
use guest_libs::chainconfig::ChainConfig;
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use tracing::error;
use walkdir::{DirEntry, WalkDir};

use crate::{BlockAndWitness, blocks_and_witnesses::WitnessGenerator};
use reth_stateless::{StatelessInput, fork_spec::ForkSpec};

/// Witness generator that produces `BlockAndWitness` fixtures for execution-spec-test fixtures.
#[derive(Debug, Clone, Default)]
pub struct ExecSpecTestBlocksAndWitnessBuilder {
    input_folder: Option<PathBuf>,
    tag: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl ExecSpecTestBlocksAndWitnessBuilder {
    const TEMP_EEST_FIXTURES_PATH: &str = "./zkevm-fixtures";

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

        // delete_eest_folder indicates if the EEST folder will be automatically deleted after witness generation.
        // If this folder was explicitly provided, we do not delete it.
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
            (PathBuf::from(&Self::TEMP_EEST_FIXTURES_PATH), true)
        };

        Ok(ExecSpecTestBlocksAndWitnesses {
            directory_path,
            include,
            exclude,
            delete_eest_folder,
        })
    }
}

/// Witness generator that produces `BlockAndWitness` fixtures for EEST fixtures.
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
                Err(e) => error!(
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
    async fn generate(&self) -> Result<Vec<BlockAndWitness>> {
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
                    error!("Failed to load test case from {}: {e}", path.display());
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
                .collect();
            tests.extend(file_tests);
        }

        let bws: Result<Vec<_>> = tests
            .par_iter()
            .map(|(name, case)| {
                Ok(BlockAndWitness {
                    name: name.to_string(),
                    block_and_witness: run_case(case)?
                        .into_iter()
                        .next_back()
                        .map(|(recovered_block, witness)| StatelessInput {
                            block: recovered_block.into_block(),
                            witness,
                        })
                        .ok_or_else(|| anyhow!("No target block found for test case {}", name))?,
                    chain_config: ChainConfig::Test(ForkSpec::from(case.network)),
                })
            })
            .collect();

        bws
    }

    /// Generates `BlockAndWitness` fixtures from EEST test cases and writes them to the specified path.
    ///
    /// This method processes all matching EEST test cases, generates the corresponding
    /// witness data, and writes each fixture as a separate JSON file in the output directory.
    ///
    /// # Arguments
    /// * `path` - The directory path where JSON fixture files will be written
    ///
    /// # Returns
    /// The number of fixture files successfully generated and written
    ///
    /// # Errors
    /// Returns an error if fixture generation fails, serialization fails, or file writing fails.
    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let bws = self.generate().await?;
        for bw in &bws {
            let output_path = path.join(format!("{}.json", bw.name));
            let output_data = serde_json::to_string_pretty(&bw)
                .with_context(|| format!("Failed to serialize fixture: {}", bw.name))?;

            std::fs::write(&output_path, output_data)
                .with_context(|| format!("Failed to write fixture to: {output_path:?}"))?;
        }
        Ok(bws.len())
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
    use flate2::bufread::GzDecoder;
    use tar::Archive;

    use super::*;
    use std::{fs::File, str::FromStr};

    fn decompress_eest_release(dest_dir: &Path) -> Result<()> {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/zkevm_fixtures_v0.1.0.tar.gz");
        let file = File::open(path)?;
        let buf_reader = std::io::BufReader::new(file);
        let tar = GzDecoder::new(buf_reader);
        let mut archive = Archive::new(tar);
        archive.unpack(dest_dir)?;
        Ok(())
    }

    fn prepare_downgraded_eest_fixtures(target_path: &Path) -> Result<()> {
        let decompress_dir = tempfile::tempdir()?;
        let decompress_path = decompress_dir.path();
        decompress_eest_release(decompress_path)?;

        let single_fixture_path =
            PathBuf::from_str("fixtures/blockchain_tests/zkevm/worst_compute/worst_jumps.json")?;
        std::fs::create_dir_all(target_path.join(single_fixture_path.parent().unwrap()))?;
        std::fs::copy(
            decompress_path
                .join("zkevm-fixtures")
                .join(&single_fixture_path),
            target_path.join(&single_fixture_path),
        )?;
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_input_folder() -> Result<()> {
        let target_dir = tempfile::tempdir()?;
        let target_path = target_dir.path();
        prepare_downgraded_eest_fixtures(target_path)?;

        let wg = ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(target_path.to_path_buf())?
            .build()?;

        // The worst_jumps.json suite has two fixtures.
        assert_eq!(
            wg.generate().await?.len(),
            2,
            "Only two fixtures are expected for the worst_jumps EEST fixture"
        );

        // Then the `input_folder` is used, the folder must not be deleted.
        drop(wg);
        assert!(
            target_path.exists(),
            "Directory should still exist after drop"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_filters() -> Result<()> {
        let target_dir = tempfile::tempdir()?;
        let target_path = target_dir.path();
        prepare_downgraded_eest_fixtures(target_path)?;

        let bw_with_include = ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(target_path.to_path_buf())?
            .with_includes(vec!["Prague".to_string()])
            .build()?
            .generate()
            .await?;
        assert_eq!(
            bw_with_include.len(),
            1,
            "Only one fixture should match the include filter"
        );
        assert!(
            bw_with_include[0].name.contains("Prague"),
            "The fixture should contain 'Prague' in its name"
        );

        let bw_with_exclude = ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(target_path.to_path_buf())?
            .with_excludes(vec!["Prague".to_string()])
            .build()?
            .generate()
            .await?;
        assert_eq!(
            bw_with_exclude.len(),
            1,
            "Only one fixture should match the exclude filter"
        );
        assert!(
            !bw_with_exclude[0].name.contains("Prague"),
            "The fixture should not contain 'Prague' in its name"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_to_path() -> Result<()> {
        let target_dir = tempfile::tempdir()?;
        let target_path = target_dir.path();
        prepare_downgraded_eest_fixtures(target_path)?;

        let wg = ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(target_path.to_path_buf())?
            .build()?;

        let generation_dir = tempfile::tempdir()?;
        let generation_path = generation_dir.path();
        let count = wg.generate_to_path(generation_path).await?;
        assert_eq!(
            count, 2,
            "Only two fixtures are expected for the worst_jumps EEST fixture"
        );
        assert_eq!(
            generation_path.read_dir()?.count(),
            2,
            "There should be two generated fixture files in the output directory"
        );

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "slow-tests")]
    async fn test_generate_latest_release() -> Result<()> {
        let target_dir = tempfile::tempdir()?;
        let target_path = target_dir.path();
        decompress_eest_release(target_path)?;

        let mut bw = ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(target_path.join("zkevm-fixtures"))?
            .with_includes(vec!["Prague".to_string()])
            .build()?;

        let generated = bw.generate().await?;
        assert!(
            !generated.is_empty(),
            "Expected to generate at least one fixture from the latest EEST release"
        );

        // Simulate that the EEST fixures were downloaded using the script.
        bw.delete_eest_folder = true;

        // Since we downloaded using the script, the temporary directory of EEST fixtures created by the script
        // should be deleted when the `ExecSpecTestBlocksAndWitnesses` is dropped.
        drop(bw);
        assert!(
            !PathBuf::from(ExecSpecTestBlocksAndWitnessBuilder::TEMP_EEST_FIXTURES_PATH).exists(),
            "Directory should be deleted after drop"
        );

        Ok(())
    }
}
