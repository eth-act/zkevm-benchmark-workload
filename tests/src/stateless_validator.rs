#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};
    use tempfile::tempdir;

    use benchmark_runner::{
        guest_programs::{self},
        Action,
    };
    use ere_dockerized::ErezkVM;
    use witness_generator::{
        eest_generator::ExecSpecTestBlocksAndWitnessBuilder, WitnessGenerator,
    };

    use crate::utils::{
        assert_executions_crashed, assert_executions_successful, assert_proving_successful,
        run_guest, untar,
    };

    const TARGET_ZKVMS: [ErezkVM; 1] = [ErezkVM::SP1]; //, ErezkVM::Risc0];

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
        let inputs = guest_programs::stateless_validator_inputs(
            &bench_fixtures_dir
                .path()
                .join("mainnet-zkevm-fixtures-input"),
        )
        .unwrap();
        run_guest(
            "stateless-validator",
            &TARGET_ZKVMS,
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        assert_executions_successful(output_folder.path());
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
        let inputs = guest_programs::stateless_validator_inputs(bench_fixtures_dir.path()).unwrap();
        run_guest(
            "stateless-validator",
            &TARGET_ZKVMS,
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        assert_executions_crashed(output_folder.path());
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
        let inputs = guest_programs::stateless_validator_inputs(bench_fixtures_dir.path()).unwrap();
        run_guest(
            "stateless-validator",
            &TARGET_ZKVMS,
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        match action {
            Action::Prove => assert_proving_successful(output_folder.path()),
            Action::Execute => assert_executions_successful(output_folder.path()),
        }
    }
}
