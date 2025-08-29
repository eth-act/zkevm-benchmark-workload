#[cfg(test)]
mod tests {
    use ere_dockerized::ErezkVM;
    use std::{env, path::PathBuf};
    use tempfile::tempdir;

    use benchmark_runner::{
        runner::Action,
        stateless_validator::{self, BlockMetadata},
    };
    use witness_generator::{
        eest_generator::ExecSpecTestBlocksAndWitnessBuilder, WitnessGenerator,
    };

    use crate::utils::{
        assert_executions_crashed, assert_executions_successful, assert_proving_successful,
        get_env_zkvm_or_default, run_guest, untar,
    };

    #[tokio::test]
    async fn prove_empty_block() {
        empty_block(Action::Prove).await;
    }

    #[tokio::test]
    async fn execute_empty_block() {
        empty_block(Action::Execute).await;
    }

    #[tokio::test]
    async fn execute_mainnet_blocks() {
        let bench_fixtures_dir = tempdir().unwrap();
        untar(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("assets/mainnet-zkevm-fixtures-input.tar.gz"),
            bench_fixtures_dir.path(),
        );

        let output_folder = tempdir().unwrap();
        let inputs = stateless_validator::stateless_validator_inputs(
            &bench_fixtures_dir
                .path()
                .join("mainnet-zkevm-fixtures-input"),
        )
        .unwrap();
        let len_inputs = inputs.len();
        run_guest(
            "stateless-validator",
            &get_env_zkvm_or_default(vec![ErezkVM::SP1, ErezkVM::Risc0]),
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
    }

    #[tokio::test]
    async fn execute_invalid_blocks() {
        let eest_fixtures_path = PathBuf::from("assets/eest-invalid-block");
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
        let inputs =
            stateless_validator::stateless_validator_inputs(bench_fixtures_dir.path()).unwrap();
        let len_inputs = inputs.len();
        run_guest(
            "stateless-validator",
            &get_env_zkvm_or_default(vec![ErezkVM::SP1, ErezkVM::Risc0]),
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        assert_executions_crashed::<BlockMetadata>(output_folder.path(), len_inputs);
    }

    async fn empty_block(action: Action) {
        let eest_fixtures_path = PathBuf::from("assets/eest-empty-block");
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
        let inputs =
            stateless_validator::stateless_validator_inputs(bench_fixtures_dir.path()).unwrap();
        let len_inputs = inputs.len();
        run_guest(
            "stateless-validator",
            &get_env_zkvm_or_default(vec![ErezkVM::SP1, ErezkVM::Risc0]),
            inputs,
            output_folder.path(),
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
