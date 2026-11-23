//! Brings collected artifacts to the pinned tests-zkevm release.

use std::{
    num::NonZeroUsize,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use tracing::{error, info};

use crate::{
    artifact::{self, Migration, TESTS_ZKEVM_VERSION},
    config::CollectorConfig,
    export::collect_artifact_files,
};

const PROGRESS_EVERY: usize = 1_000;

#[derive(Debug, Clone)]
pub(crate) struct MigrateOptions {
    pub(crate) from_block: Option<u64>,
    pub(crate) to_block: Option<u64>,
    pub(crate) jobs: NonZeroUsize,
    pub(crate) dry_run: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct MigrationSummary {
    pub(crate) artifacts: usize,
    pub(crate) migrated: usize,
    pub(crate) current: usize,
    pub(crate) failed: Vec<PathBuf>,
}

/// Runs [`artifact::migrate_artifact`] on every artifact under `blocks/` in the block range, on
/// `jobs` threads. A failed artifact is logged and left as it was, and the run goes on.
pub(crate) fn migrate_artifacts(
    config: &CollectorConfig,
    options: &MigrateOptions,
) -> anyhow::Result<MigrationSummary> {
    let range = options.from_block.unwrap_or(0)..=options.to_block.unwrap_or(u64::MAX);
    let ranged = options.from_block.is_some() || options.to_block.is_some();
    let mut files = Vec::new();
    collect_artifact_files(&config.blocks_root(), &mut files)?;
    if ranged {
        files.retain(|path| in_range(path, &range));
    }
    files.sort();

    info!(
        artifacts = files.len(),
        from_block = options.from_block,
        to_block = options.to_block,
        jobs = options.jobs.get(),
        dry_run = options.dry_run,
        tests_zkevm = TESTS_ZKEVM_VERSION,
        "migrating artifacts"
    );

    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let migrated = AtomicUsize::new(0);
    let current = AtomicUsize::new(0);
    let failed = Mutex::new(Vec::new());
    thread::scope(|scope| {
        for _ in 0..options.jobs.get() {
            scope.spawn(|| {
                while let Some(path) = files.get(next.fetch_add(1, Ordering::Relaxed)) {
                    match artifact::migrate_artifact(path, options.dry_run) {
                        Ok(Migration::Migrated) => {
                            migrated.fetch_add(1, Ordering::Relaxed);
                        }
                        Ok(Migration::Current) => {
                            current.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(err) => {
                            error!(path = %path.display(), "failed to migrate artifact: {err:#}");
                            failed
                                .lock()
                                .unwrap_or_else(|poisoned| poisoned.into_inner())
                                .push(path.clone());
                        }
                    }
                    let done = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if done.is_multiple_of(PROGRESS_EVERY) {
                        info!(
                            done,
                            total = files.len(),
                            migrated = migrated.load(Ordering::Relaxed),
                            current = current.load(Ordering::Relaxed),
                            "migration progress"
                        );
                    }
                }
            });
        }
    });

    let mut failed = failed
        .into_inner()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    failed.sort();
    let summary = MigrationSummary {
        artifacts: files.len(),
        migrated: migrated.into_inner(),
        current: current.into_inner(),
        failed,
    };
    info!(
        artifacts = summary.artifacts,
        migrated = summary.migrated,
        current = summary.current,
        failed = summary.failed.len(),
        dry_run = options.dry_run,
        "migrated artifacts"
    );
    Ok(summary)
}

/// Whether the block number in an artifact's file name, `<number>-<hash>.json.zst`, is in
/// `range`. A file whose name doesn't parse is left out of a ranged run.
fn in_range(path: &Path, range: &RangeInclusive<u64>) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.split_once('-'))
        .and_then(|(number, _)| number.parse::<u64>().ok())
        .is_some_and(|number| range.contains(&number))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use alloy_primitives::B256;
    use stateless_validator_common::{
        HashTreeRoot as _, Sha2Hasher, SszDecode as _, SszEncode as _,
        guest::{StatelessInput, StatelessValidationResult},
    };

    use super::*;
    use crate::artifact::{
        StatelessInputArtifact, read_artifact, test_generated_input, write_artifact_atomic,
    };

    #[test]
    fn migrates_artifacts_with_the_old_root() {
        let config = test_config("old_root");
        let old = write_old_artifact(&config, 10, B256::repeat_byte(0xaa));
        let current = write_current_artifact(&config, 11, B256::repeat_byte(0xbb));
        let current_bytes = fs::read(&current).unwrap();
        assert!(read_artifact(&old).is_err());

        let summary = migrate_artifacts(&config, &options(None, None, false)).unwrap();

        assert_eq!(summary.artifacts, 2);
        assert_eq!(summary.migrated, 1);
        assert_eq!(summary.current, 1);
        assert!(summary.failed.is_empty());
        let migrated = read_artifact(&old).unwrap();
        assert_eq!(migrated.block_number, 10);
        let (input, output) = payload_of(&old);
        assert_eq!(
            output.new_payload_request_root,
            input.new_payload_request.hash_tree_root(&Sha2Hasher)
        );
        assert_eq!(fs::read(&current).unwrap(), current_bytes);
    }

    #[test]
    fn migration_changes_only_the_root_and_the_version() {
        let config = test_config("only_root");
        let generated = test_generated_input(10, B256::repeat_byte(0xaa));
        let artifact = generated_artifact(&generated);
        let path = write_current_artifact(&config, 10, B256::repeat_byte(0xaa));
        let expected = json_of(&path);
        make_old(&path);

        migrate_artifacts(&config, &options(None, None, false)).unwrap();

        assert_eq!(json_of(&path), expected);
        assert_eq!(read_artifact(&path).unwrap(), artifact);
    }

    #[test]
    fn dry_run_leaves_files_alone() {
        let config = test_config("dry_run");
        let old = write_old_artifact(&config, 10, B256::repeat_byte(0xaa));
        let old_bytes = fs::read(&old).unwrap();

        let summary = migrate_artifacts(&config, &options(None, None, true)).unwrap();

        assert_eq!(summary.migrated, 1);
        assert_eq!(fs::read(&old).unwrap(), old_bytes);
    }

    #[test]
    fn migrates_only_the_block_range() {
        let config = test_config("range");
        let paths = (10..15)
            .map(|block| write_old_artifact(&config, block, B256::repeat_byte(block as u8)))
            .collect::<Vec<_>>();

        let summary = migrate_artifacts(&config, &options(Some(11), Some(13), false)).unwrap();

        assert_eq!(summary.artifacts, 3);
        assert_eq!(summary.migrated, 3);
        for (block, path) in (10..15).zip(&paths) {
            assert_eq!(
                read_artifact(path).is_ok(),
                (11..=13).contains(&block),
                "block {block}"
            );
        }
    }

    #[test]
    fn reports_artifacts_that_stay_invalid() {
        let config = test_config("invalid");
        let good = write_old_artifact(&config, 10, B256::repeat_byte(0xaa));
        let bad = write_old_artifact(&config, 11, B256::repeat_byte(0xbb));
        edit_json(&bad, |test| {
            test["_info"]["metadata"]["witness_generator"]["blockHash"] =
                serde_json::json!(B256::repeat_byte(0xcc).to_string());
        });
        let bad_bytes = fs::read(&bad).unwrap();

        let summary = migrate_artifacts(&config, &options(None, None, false)).unwrap();

        assert_eq!(summary.migrated, 1);
        assert_eq!(summary.failed, vec![bad.clone()]);
        assert!(read_artifact(&good).is_ok());
        assert_eq!(fs::read(&bad).unwrap(), bad_bytes);
    }

    #[test]
    fn reader_names_the_fix_for_unmigrated_artifacts() {
        let config = test_config("reader_hint");
        let old = write_old_artifact(&config, 10, B256::repeat_byte(0xaa));

        let error = format!("{:#}", read_artifact(&old).unwrap_err());

        assert!(error.contains("v21.0.6 or older"), "{error}");
        assert!(error.contains("migrate"), "{error}");
    }

    /// The root of the test payload request under the pinned ere-guests. If an ere-guests bump
    /// changes it, bump `TESTS_ZKEVM_VERSION` with it, so `migrate` rewrites the old roots.
    #[test]
    fn payload_request_root_matches_tests_zkevm_version() {
        let generated = test_generated_input(10, B256::repeat_byte(0xaa));
        let (_, input) =
            StatelessInput::from_schema_prefixed_ssz(&generated.stateless_input_bytes).unwrap();

        assert_eq!(TESTS_ZKEVM_VERSION, "v21.0.7");
        assert_eq!(
            B256::from(input.new_payload_request.hash_tree_root(&Sha2Hasher)).to_string(),
            "0x8709e4250e9e5c96c8e37e75f1e80a240a52108b03da8ca8ba1f57fb9820ec69"
        );
    }

    fn options(from_block: Option<u64>, to_block: Option<u64>, dry_run: bool) -> MigrateOptions {
        MigrateOptions {
            from_block,
            to_block,
            jobs: NonZeroUsize::new(2).unwrap(),
            dry_run,
        }
    }

    fn generated_artifact(
        generated: &witness_generator_spec_cli::GeneratedInput,
    ) -> StatelessInputArtifact {
        StatelessInputArtifact::from_generated_at(
            "sepolia",
            "head",
            generated,
            "2026-11-23T00:00:00Z",
            "test-commit".to_owned(),
        )
        .unwrap()
    }

    fn write_current_artifact(
        config: &CollectorConfig,
        block_number: u64,
        block_hash: B256,
    ) -> PathBuf {
        let artifact = generated_artifact(&test_generated_input(block_number, block_hash));
        write_artifact_atomic(&config.blocks_root(), &artifact)
            .unwrap()
            .path
    }

    /// An artifact as the collector wrote it before v21.0.7: a different root and no
    /// `testsZkevmVersion`.
    fn write_old_artifact(
        config: &CollectorConfig,
        block_number: u64,
        block_hash: B256,
    ) -> PathBuf {
        let path = write_current_artifact(config, block_number, block_hash);
        make_old(&path);
        path
    }

    fn make_old(path: &Path) {
        edit_json(path, |test| {
            let payload = &mut test["engineNewPayloads"][0];
            let bytes = hex_decode(payload["statelessOutputBytes"].as_str().unwrap());
            let mut output = StatelessValidationResult::from_ssz_bytes(&bytes).unwrap();
            output.new_payload_request_root = [0x11; 32];
            payload["statelessOutputBytes"] = serde_json::json!(hex_encode(&output.to_ssz()));
            test["_info"]["metadata"]["witness_generator"]
                .as_object_mut()
                .unwrap()
                .remove("testsZkevmVersion");
        });
    }

    fn edit_json(path: &Path, edit: impl FnOnce(&mut serde_json::Value)) {
        let mut value: serde_json::Value = serde_json::from_slice(&json_of(path)).unwrap();
        edit(value.as_object_mut().unwrap().values_mut().next().unwrap());
        let mut json = serde_json::to_vec_pretty(&value).unwrap();
        json.push(b'\n');
        fs::write(path, zstd::bulk::compress(&json, 3).unwrap()).unwrap();
    }

    fn json_of(path: &Path) -> Vec<u8> {
        zstd::stream::decode_all(fs::File::open(path).unwrap()).unwrap()
    }

    fn payload_of(path: &Path) -> (StatelessInput, StatelessValidationResult) {
        let value: serde_json::Value = serde_json::from_slice(&json_of(path)).unwrap();
        let payload = &value.as_object().unwrap().values().next().unwrap()["engineNewPayloads"][0];
        let input = hex_decode(payload["statelessInputBytes"].as_str().unwrap());
        let output = hex_decode(payload["statelessOutputBytes"].as_str().unwrap());
        (
            StatelessInput::from_schema_prefixed_ssz(&input).unwrap().1,
            StatelessValidationResult::from_ssz_bytes(&output).unwrap(),
        )
    }

    fn hex_decode(value: &str) -> Vec<u8> {
        alloy_primitives::hex::decode(value.trim_start_matches("0x")).unwrap()
    }

    fn hex_encode(bytes: &[u8]) -> String {
        format!("0x{}", alloy_primitives::hex::encode(bytes))
    }

    fn test_config(name: &str) -> CollectorConfig {
        let out_root = std::env::temp_dir().join(format!(
            "witness-generator-spec-cli-migrate-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&out_root);
        CollectorConfig::from_toml_str(&format!(
            "network = \"sepolia\"\ncl_url = \"http://cl\"\nel_url = \"http://el\"\nout_root = \"{}\"\n",
            out_root.display()
        ))
        .unwrap()
    }
}
