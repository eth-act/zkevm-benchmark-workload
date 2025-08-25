//! Integration tests

#[cfg(test)]
mod tests {
    use flate2::bufread::GzDecoder;
    use std::{
        env,
        fs::File,
        path::{Path, PathBuf},
    };
    use tempfile::tempdir;

    use benchmark_runner::{
        get_zkvm_instances,
        guest_programs::{self, BlockMetadata},
        run_benchmark, Action, RunConfig,
    };
    use ere_dockerized::ErezkVM;
    use tar::Archive;
    use walkdir::WalkDir;
    use witness_generator::{
        eest_generator::ExecSpecTestBlocksAndWitnessBuilder, WitnessGenerator,
    };
    use zkevm_metrics::{BenchmarkRun, ExecutionMetrics};
    use zkvm_interface::ProverResourceType;

    const ZKVMS: [ErezkVM; 1] = [ErezkVM::SP1]; //, ErezkVM::Risc0];

    #[tokio::test]
    async fn execute_mainnet_blocks() {
        let bench_fixtures_dir = tempdir().unwrap();
        untar(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("mainnet-zkevm-fixtures-input.tar.gz"),
            bench_fixtures_dir.path(),
        );

        let output_folder = tempdir().unwrap();
        run_stateless_validator(
            &bench_fixtures_dir
                .path()
                .join("mainnet-zkevm-fixtures-input"),
            output_folder.path(),
        );
        assert_executions_successful(output_folder.path());
    }

    #[tokio::test]
    async fn execute_invalid_blocks() {
        let eest_fixtures_path = PathBuf::from("eest-invalid-block-fixtures");
        let bench_fixtures_dir = tempdir().unwrap();
        ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(eest_fixtures_path)
            .unwrap()
            .build()
            .unwrap()
            .generate_to_path(bench_fixtures_dir.path())
            .await
            .unwrap();

        let output_folder = tempdir().unwrap();
        run_stateless_validator(bench_fixtures_dir.path(), output_folder.path());
        assert_executions_crashed(output_folder.path());
    }

    fn run_stateless_validator(bench_fixtures_path: &Path, output_folder: &Path) {
        let inputs = guest_programs::stateless_validator_inputs(bench_fixtures_path).unwrap();
        let config = RunConfig {
            output_folder: output_folder.to_path_buf(),
            action: Action::Execute,
            force_rerun: true,
        };
        let instances = get_zkvm_instances(
            &ZKVMS,
            &PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join("ere-guests"),
            Path::new("stateless-validator"),
            ProverResourceType::Cpu,
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

    fn assert_executions_crashed(metrics_folder_path: &Path) {
        assert_execution_status(metrics_folder_path, |exec| {
            matches!(exec, ExecutionMetrics::Crashed { .. })
        });
    }

    fn assert_executions_successful(metrics_folder_path: &Path) {
        assert_execution_status(metrics_folder_path, |exec| {
            matches!(exec, ExecutionMetrics::Success { .. })
        });
    }

    fn assert_execution_status<F>(output_path: &Path, predicate: F)
    where
        F: Fn(&ExecutionMetrics) -> bool,
    {
        WalkDir::new(output_path)
            .min_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .for_each(|entry| {
                let result = BenchmarkRun::<BlockMetadata>::from_path(entry.path()).unwrap();
                let execution = result.execution.unwrap();
                assert!(
                    predicate(&execution),
                    "Unexpected execution status for: {}",
                    entry.path().display()
                );
            });
    }

    fn untar(path: &Path, dest_dir: &Path) {
        let file = File::open(path).unwrap();
        let buf_reader = std::io::BufReader::new(file);
        let tar = GzDecoder::new(buf_reader);
        let mut archive = Archive::new(tar);
        archive.unpack(dest_dir).unwrap();
    }
}
