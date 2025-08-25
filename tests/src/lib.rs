//! Integration tests

#[cfg(test)]
mod tests {
    use std::{
        env,
        path::{Path, PathBuf},
    };

    use benchmark_runner::{
        get_zkvm_instances,
        guest_programs::{self, BlockMetadata},
        run_benchmark, Action, RunConfig,
    };
    use ere_dockerized::ErezkVM;
    use walkdir::WalkDir;
    use witness_generator::{
        eest_generator::ExecSpecTestBlocksAndWitnessBuilder, WitnessGenerator,
    };
    use zkevm_metrics::BenchmarkRun;
    use zkvm_interface::{zkVM, ProverResourceType};

    const ZKVMS: [ErezkVM; 1] = [ErezkVM::SP1]; //, ErezkVM::Risc0];

    #[tokio::test]
    async fn execute_invalid_blocks() {
        let eest_fixtures_path = PathBuf::from("eest-invalid-block-fixtures");
        let bench_fixtures_dir = tempfile::tempdir().unwrap();
        ExecSpecTestBlocksAndWitnessBuilder::default()
            .with_input_folder(eest_fixtures_path)
            .unwrap()
            .build()
            .unwrap()
            .generate_to_path(bench_fixtures_dir.path())
            .await
            .unwrap();

        let inputs = guest_programs::stateless_validator_inputs(bench_fixtures_dir.path()).unwrap();
        let output_folder = tempfile::tempdir().unwrap();
        let config = RunConfig {
            output_folder: output_folder.path().to_path_buf(),
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

            let zkvm_folder_name = format!("{}-v{}", zkvm.name(), zkvm.sdk_version());
            let zkvm_folder_path = output_folder.path().join(zkvm_folder_name);
            assert!(std::fs::exists(&zkvm_folder_path).unwrap());
            WalkDir::new(zkvm_folder_path)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
                .for_each(|entry| {
                    let result = BenchmarkRun::<BlockMetadata>::from_path(entry.path()).unwrap();
                    assert!(
                        matches!(
                            result.execution.unwrap(),
                            zkevm_metrics::ExecutionMetrics::Crashed { .. }
                        ),
                        "Expected execution to be crashed for file: {}",
                        entry.path().display()
                    );
                });
        }

        assert!(
            std::fs::exists(output_folder.path().join("hardware.json")).unwrap(),
            "hardware.json file must exist"
        );
    }
}
