//! Generate benchmark fixtures for EEST blockchain tests.

use async_trait::async_trait;
use ef_tests::{
    Error as EFTestError, cases::blockchain_test::BlockchainTestCase, models::BlockchainTest,
};
use rayon::prelude::*;
use reth_chainspec::{Chain, blob_params_to_schedule, create_chain_config};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::{error, info, warn};
use walkdir::{DirEntry, WalkDir};

use crate::{Fixture, FixtureGenerator, Result, StatelessValidationFixture, WGError};
use stateless::StatelessInput;

/// Witness generator that produces `BlockAndWitness` fixtures for execution-spec-test fixtures.
#[derive(Debug, Clone, Default)]
pub struct EESTFixtureGeneratorBuilder {
    input_folder: Option<PathBuf>,
    tag: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl EESTFixtureGeneratorBuilder {
    const TEMP_EEST_FIXTURES_PATH: &str = "./zkevm-fixtures";

    /// Configures which execution-spec-test version tag to download.
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Specifies a local directory containing pre-downloaded EEST fixtures, skipping automatic download.
    pub fn with_input_folder(mut self, path: PathBuf) -> Result<Self> {
        if !path.exists() {
            return Err(WGError::EestPathNotFound(path.display().to_string()));
        }
        if !path.is_dir() {
            return Err(WGError::EestPathNotDirectory(path.display().to_string()));
        }
        let canonical_path = path.canonicalize()?;

        self.input_folder = Some(canonical_path);
        Ok(self)
    }

    /// Filters to include only test cases whose names contain any of the specified substrings.
    pub fn with_includes(mut self, includes: Vec<String>) -> Self {
        self.include = Some(includes);
        self
    }

    /// Filters to exclude test cases whose names contain any of the specified substrings.
    pub fn with_excludes(mut self, exclude: Vec<String>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    /// Constructs the generator, downloading EEST fixtures if no local path was specified.
    pub async fn build(self) -> Result<EESTFixtureGenerator> {
        let input_folder = self.input_folder;
        let tag = self.tag;
        let include = self.include.unwrap_or_default();
        let exclude = self.exclude.unwrap_or_default();

        // delete_eest_folder indicates if the EEST folder will be automatically deleted after witness generation.
        // If this folder was explicitly provided, we do not delete it.
        let (directory_path, delete_eest_folder) = if let Some(input_folder) = input_folder {
            (input_folder, false)
        } else {
            let dest = PathBuf::from(Self::TEMP_EEST_FIXTURES_PATH);
            crate::eest_downloader::download_and_extract(tag.as_deref(), &dest).await?;
            (dest, true)
        };

        Ok(EESTFixtureGenerator {
            eest_fixtures: directory_path,
            filter_include: include,
            filter_exclude: exclude,
            delete_eest_fixtures: delete_eest_folder,
        })
    }
}

/// Witness generator that produces `BlockAndWitness` fixtures for EEST fixtures.
#[derive(Debug, Clone)]
pub struct EESTFixtureGenerator {
    eest_fixtures: PathBuf,
    filter_include: Vec<String>,
    filter_exclude: Vec<String>,
    delete_eest_fixtures: bool,
}

impl Drop for EESTFixtureGenerator {
    fn drop(&mut self) {
        if self.delete_eest_fixtures && self.eest_fixtures.exists() {
            match std::fs::remove_dir_all(&self.eest_fixtures) {
                Ok(_) => {}
                Err(e) => error!(
                    "Failed to remove directory {}: {}",
                    self.eest_fixtures.display(),
                    e
                ),
            }
        }
    }
}

#[async_trait]
impl FixtureGenerator for EESTFixtureGenerator {
    /// Streams matching EEST blockchain tests to disk without retaining every generated fixture.
    async fn generate_to_path(&self, path: &Path) -> Result<usize> {
        let suite_path = self.suite_path()?;
        let test_file_paths = find_all_files_with_extension(&suite_path, ".json");
        info!(
            "Generating EEST fixtures from {} source files",
            test_file_paths.len()
        );

        let mut count = 0;
        for test_path in test_file_paths {
            let tests =
                load_filtered_tests(&test_path, &self.filter_include, &self.filter_exclude)?;
            let test_count = tests.len();
            if test_count == 0 {
                continue;
            }

            let generated_count = tests
                .par_iter()
                .map(|(name, case)| {
                    let Some(fixture) = gen_fixture_or_skip(name, case)? else {
                        return Ok(0);
                    };

                    write_fixture(fixture.as_ref(), path)?;
                    Ok(1)
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .sum::<usize>();
            count += generated_count;
        }

        Ok(count)
    }

    /// Loads EEST blockchain tests, applies include/exclude filters, and generates typed witness fixtures in parallel.
    async fn generate(&self) -> Result<Vec<Box<dyn Fixture>>> {
        let suite_path = self.suite_path()?;
        let test_file_paths = find_all_files_with_extension(&suite_path, ".json");
        let mut tests: Vec<(String, BlockchainTest)> = Vec::new();
        for path in test_file_paths {
            tests.extend(load_filtered_tests(
                &path,
                &self.filter_include,
                &self.filter_exclude,
            )?);
        }

        let bws = tests
            .par_iter()
            .map(|(name, case)| gen_fixture_or_skip(name, case))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(bws)
    }
}

impl EESTFixtureGenerator {
    fn suite_path(&self) -> Result<PathBuf> {
        let suite_path = self.eest_fixtures.join("fixtures/blockchain_tests");

        if !suite_path.exists() {
            return Err(WGError::TestSuitePathNotFound(
                suite_path.display().to_string(),
            ));
        }

        Ok(suite_path)
    }
}

fn load_filtered_tests(
    path: &Path,
    filter_include: &[String],
    filter_exclude: &[String],
) -> Result<Vec<(String, BlockchainTest)>> {
    let raw_tests = load_raw_tests(path)?;

    raw_tests
        .into_iter()
        .filter(|(name, _)| {
            let name = display_test_name(name);
            matches_filters(name, filter_include, filter_exclude)
        })
        .map(|(name, case)| {
            let name = display_test_name(&name).to_string();
            let case = serde_json::from_value(case).map_err(|error| {
                test_case_load_error(
                    path,
                    EFTestError::CouldNotDeserialize {
                        path: path.into(),
                        error,
                    },
                )
            })?;

            Ok((name, case))
        })
        .collect()
}

fn load_raw_tests(path: &Path) -> Result<BTreeMap<String, serde_json::Value>> {
    let contents = fs::read_to_string(path).map_err(|error| {
        test_case_load_error(
            path,
            EFTestError::Io {
                path: path.into(),
                error,
            },
        )
    })?;

    serde_json::from_str(&contents).map_err(|error| {
        test_case_load_error(
            path,
            EFTestError::CouldNotDeserialize {
                path: path.into(),
                error,
            },
        )
    })
}

fn display_test_name(name: &str) -> &str {
    name.split('/').next_back().unwrap_or(name)
}

fn matches_filters(name: &str, filter_include: &[String], filter_exclude: &[String]) -> bool {
    !filter_exclude.iter().any(|filter| name.contains(filter))
        && filter_include.iter().all(|filter| name.contains(filter))
}

fn test_case_load_error(path: &Path, source: EFTestError) -> WGError {
    WGError::TestCaseLoadError {
        path: path.display().to_string(),
        source: Box::new(source),
    }
}

fn write_fixture(fixture: &dyn Fixture, path: &Path) -> Result<()> {
    let output_path = path.join(format!("{}.json", fixture.name()));
    let mut buf = Vec::new();
    let mut serializer = serde_json::Serializer::pretty(&mut buf);
    erased_serde::serialize(fixture, &mut serializer).map_err(|error| {
        WGError::FixtureSerializationError {
            name: fixture.name().to_owned(),
            source: error,
        }
    })?;

    std::fs::write(&output_path, buf).map_err(|error| WGError::FixtureWriteError {
        path: output_path.display().to_string(),
        source: error,
    })
}

fn gen_fixture_or_skip(name: &str, case: &BlockchainTest) -> Result<Option<Box<dyn Fixture>>> {
    match gen_fixture(name, case) {
        Ok(fixture) => Ok(Some(fixture)),
        Err(WGError::NoTargetBlock(name)) => {
            warn!("Skipping EEST test case {name}: no executed block/witness was produced");
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

fn gen_fixture(name: &str, case: &BlockchainTest) -> Result<Box<dyn Fixture>> {
    let spec = case.network.to_chain_spec();
    let chain_config = create_chain_config(
        Some(Chain::mainnet()),
        &spec.hardforks,
        spec.deposit_contract.map(|dc| dc.address),
        blob_params_to_schedule(&spec.blob_params, &spec.hardforks),
    );

    let (block, witness) = BlockchainTestCase::run_single_case(name, case)
        .map_err(|e| WGError::TestCaseExecutionError {
            source: Box::new(e),
        })?
        .into_iter()
        .next_back()
        .map(|(block, witnesses)| (block.into_block(), witnesses))
        .ok_or_else(|| WGError::NoTargetBlock(name.to_owned()))?;

    let success = case
        .blocks
        .iter()
        .next_back()
        .unwrap()
        .expect_exception
        .is_none();

    let res: Box<dyn Fixture> = Box::new(StatelessValidationFixture {
        name: name.to_owned(),
        stateless_input: StatelessInput {
            block,
            witness,
            chain_config,
        },
        success,
    });

    Ok(res)
}

fn find_all_files_with_extension(path: &Path, extension: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(extension))
        .map(DirEntry::into_path)
        .collect()
}
