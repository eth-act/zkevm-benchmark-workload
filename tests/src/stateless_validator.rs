#[cfg(test)]
mod tests {
    use benchmark_runner::{
        runner::Action,
        stateless_validator::{self, BlockMetadata, ExecutionClient},
    };
    use ere_dockerized::zkVMKind;
    use std::{env, path::PathBuf};
    use tempfile::{tempdir, TempDir};
    use witness_generator::{eest_generator::EESTFixtureGeneratorBuilder, FixtureGenerator};

    use crate::utils::{
        assert_executions_successful, assert_proving_successful, filter_el_zkvm_pairs_from_env,
        run_guest, untar,
    };

    #[tokio::test(flavor = "multi_thread")]
    async fn prove_empty_block() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, zkVMKind::SP1),
            (ExecutionClient::Reth, zkVMKind::Risc0),
            (ExecutionClient::Reth, zkVMKind::OpenVM),
            (ExecutionClient::Reth, zkVMKind::Pico),
            (ExecutionClient::Reth, zkVMKind::Airbender),
            (ExecutionClient::Ethrex, zkVMKind::SP1),
            (ExecutionClient::Ethrex, zkVMKind::Risc0),
            // (ExecutionClient::Ethrex, zkVMKind::OpenVM), // See https://github.com/eth-act/ere/issues/168
            // (ExecutionClient::Ethrex, zkVMKind::Pico), // See https://github.com/eth-act/ere/issues/174
        ]);
        empty_block(Action::Prove, &el_zkvms).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_empty_block() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, zkVMKind::SP1),
            (ExecutionClient::Reth, zkVMKind::Risc0),
            (ExecutionClient::Reth, zkVMKind::OpenVM),
            (ExecutionClient::Reth, zkVMKind::Pico),
            (ExecutionClient::Reth, zkVMKind::Zisk),
            (ExecutionClient::Reth, zkVMKind::Airbender),
            (ExecutionClient::Ethrex, zkVMKind::SP1),
            (ExecutionClient::Ethrex, zkVMKind::Risc0),
            (ExecutionClient::Ethrex, zkVMKind::Zisk),
            // (ExecutionClient::Ethrex, zkVMKind::OpenVM), // See https://github.com/eth-act/ere/issues/168
            // (ExecutionClient::Ethrex, zkVMKind::Pico), // See https://github.com/eth-act/ere/issues/174
        ]);
        empty_block(Action::Execute, &el_zkvms).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_mainnet_blocks() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, zkVMKind::SP1),
            (ExecutionClient::Reth, zkVMKind::Risc0),
            (ExecutionClient::Reth, zkVMKind::OpenVM),
            (ExecutionClient::Reth, zkVMKind::Pico),
            (ExecutionClient::Reth, zkVMKind::Zisk),
            (ExecutionClient::Reth, zkVMKind::Airbender),
            (ExecutionClient::Ethrex, zkVMKind::SP1),
            (ExecutionClient::Ethrex, zkVMKind::Risc0),
            (ExecutionClient::Ethrex, zkVMKind::Zisk),
        ]);

        for (el, zkvm) in el_zkvms {
            let bench_fixtures_dir = tempdir().unwrap();
            untar(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/mainnet-zkevm-fixtures-input.tar.gz"),
                bench_fixtures_dir.path(),
            );
            let input_folder = &bench_fixtures_dir
                .path()
                .join("mainnet-zkevm-fixtures-input");

            let output_folder = OutputDir::new().unwrap();
            let inputs = stateless_validator::stateless_validator_inputs(input_folder, el).unwrap();
            let len_inputs = inputs.len();
            assert_eq!(len_inputs, 15);

            let el_str = el.as_ref().to_lowercase();
            run_guest(
                &format!("stateless-validator/{el_str}"),
                &[zkvm],
                inputs,
                output_folder.path(),
                Some(el_str),
                Action::Execute,
            );
            assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_invalid_block() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, zkVMKind::SP1),
            (ExecutionClient::Reth, zkVMKind::Risc0),
            (ExecutionClient::Reth, zkVMKind::OpenVM),
            (ExecutionClient::Reth, zkVMKind::Pico),
            (ExecutionClient::Reth, zkVMKind::Zisk),
            (ExecutionClient::Reth, zkVMKind::Airbender),
            (ExecutionClient::Ethrex, zkVMKind::SP1),
            (ExecutionClient::Ethrex, zkVMKind::Risc0),
            (ExecutionClient::Ethrex, zkVMKind::Zisk),
        ]);
        for (el, zkvm) in el_zkvms {
            let eest_fixtures_path = PathBuf::from("assets/eest-invalid-block");
            let bench_fixtures_dir = tempdir().unwrap();
            EESTFixtureGeneratorBuilder::default()
                .with_input_folder(eest_fixtures_path)
                .unwrap()
                .build()
                .unwrap()
                .generate_to_path(bench_fixtures_dir.path())
                .await
                .unwrap();

            let output_folder = OutputDir::new().unwrap();
            let inputs =
                stateless_validator::stateless_validator_inputs(bench_fixtures_dir.path(), el)
                    .unwrap();

            let len_inputs = inputs.len();
            assert_eq!(len_inputs, 1);

            let el_str = el.as_ref().to_lowercase();
            run_guest(
                &format!("stateless-validator/{el_str}"),
                &[zkvm],
                inputs,
                output_folder.path(),
                Some(el_str),
                Action::Execute,
            );
            assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
        }
    }

    async fn empty_block(action: Action, el_zkvms: &[(ExecutionClient, zkVMKind)]) {
        for (el, zkvm) in el_zkvms {
            let eest_fixtures_path = PathBuf::from("assets/eest-empty-block");
            let bench_fixtures_dir = tempdir().unwrap();
            EESTFixtureGeneratorBuilder::default()
                .with_input_folder(eest_fixtures_path)
                .unwrap()
                .build()
                .unwrap()
                .generate_to_path(bench_fixtures_dir.path())
                .await
                .unwrap();

            let output_folder = OutputDir::new().unwrap();
            let inputs =
                stateless_validator::stateless_validator_inputs(bench_fixtures_dir.path(), *el)
                    .unwrap();

            let len_inputs = inputs.len();
            assert_eq!(len_inputs, 1);

            run_guest(
                &format!("stateless-validator/{}", el.as_ref().to_lowercase()),
                &[*zkvm],
                inputs,
                output_folder.path(),
                Some(el.as_ref().to_lowercase()),
                action,
            );
            match action {
                Action::Prove => {
                    assert_proving_successful::<BlockMetadata>(output_folder.path(), len_inputs)
                }
                Action::Execute => {
                    assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
                }
            }
        }
    }
    struct OutputDir {
        path: PathBuf,
        // When OutputDir is dropped, the temp dir (if any) meant to be deleted
        _temp_dir: Option<TempDir>,
    }

    impl OutputDir {
        /// Create an output directory from env var or as a temp dir
        fn new() -> Result<Self, std::io::Error> {
            if let Ok(base_dir) = env::var("WORKLOAD_OUTPUT_DIR") {
                std::fs::create_dir_all(&base_dir)?;
                Ok(Self {
                    path: PathBuf::from(base_dir).canonicalize()?,
                    _temp_dir: None,
                })
            } else {
                let temp_dir = tempdir()?;
                let path = temp_dir.path().to_path_buf();
                Ok(Self {
                    path,
                    _temp_dir: Some(temp_dir),
                })
            }
        }

        /// Get the path to the directory
        fn path(&self) -> &PathBuf {
            &self.path
        }
    }
}
