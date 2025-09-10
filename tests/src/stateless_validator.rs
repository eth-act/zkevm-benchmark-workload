#[cfg(test)]
mod tests {
    use benchmark_runner::{
        runner::Action,
        stateless_validator::{self, BlockMetadata, ExecutionClient},
    };
    use ere_dockerized::ErezkVM;
    use std::{env, path::PathBuf};
    use tempfile::tempdir;
    use witness_generator::{
        eest_generator::ExecSpecTestBlocksAndWitnessBuilder, WitnessGenerator,
    };

    use crate::utils::{
        assert_executions_successful, assert_proving_successful, filter_el_zkvm_pairs_from_env,
        get_env_zkvm_or_default, run_guest, untar,
    };

    #[tokio::test]
    async fn prove_empty_block() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, ErezkVM::SP1),
            (ExecutionClient::Reth, ErezkVM::Risc0),
            (ExecutionClient::Reth, ErezkVM::OpenVM),
            // (ExecutionClient::Reth, ErezkVM::Pico), // See https://github.com/eth-act/ere/issues/173
            (ExecutionClient::Ethrex, ErezkVM::SP1),
            // (ExecutionClient::Ethrex, ErezkVM::Risc0), // See https://github.com/eth-act/ere/issues/121
            // (ExecutionClient::Ethrex, ErezkVM::OpenVM), // See https://github.com/eth-act/ere/issues/168
            // (ExecutionClient::Ethrex, ErezkVM::Pico), // See https://github.com/eth-act/ere/issues/174
        ]);
        empty_block(Action::Prove, &el_zkvms).await;
    }

    #[tokio::test]
    async fn execute_empty_block() {
        let el_zkvms = filter_el_zkvm_pairs_from_env(vec![
            (ExecutionClient::Reth, ErezkVM::SP1),
            (ExecutionClient::Reth, ErezkVM::Risc0),
            (ExecutionClient::Reth, ErezkVM::OpenVM),
            (ExecutionClient::Reth, ErezkVM::Pico),
            (ExecutionClient::Ethrex, ErezkVM::SP1),
            // (ExecutionClient::Ethrex, ErezkVM::Risc0), // See https://github.com/eth-act/ere/issues/121
            // (ExecutionClient::Ethrex, ErezkVM::OpenVM), // See https://github.com/eth-act/ere/issues/168
            // (ExecutionClient::Ethrex, ErezkVM::Pico), // See https://github.com/eth-act/ere/issues/174
        ]);
        empty_block(Action::Execute, &el_zkvms).await;
    }

    #[tokio::test]
    async fn execute_mainnet_blocks() {
        let zkvms = get_env_zkvm_or_default(vec![
            ErezkVM::SP1,
            ErezkVM::Risc0,
            ErezkVM::OpenVM,
            ErezkVM::Pico,
        ]);
        for zkvm in &zkvms {
            println!("Using zkVM: {zkvm}");
            let bench_fixtures_dir = tempdir().unwrap();
            untar(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/mainnet-zkevm-fixtures-input.tar.gz"),
                bench_fixtures_dir.path(),
            );
            let input_folder = &bench_fixtures_dir
                .path()
                .join("mainnet-zkevm-fixtures-input");

            let output_folder = tempdir().unwrap();
            let inputs = stateless_validator::stateless_validator_inputs(
                input_folder,
                ExecutionClient::Reth,
            )
            .unwrap();
            let len_inputs = inputs.len();
            assert_eq!(len_inputs, 15);
            run_guest(
                "stateless-validator/reth",
                &[*zkvm],
                inputs,
                output_folder.path(),
                Action::Execute,
            );
            assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
        }
    }

    #[tokio::test]
    async fn execute_invalid_block() {
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
        let inputs = stateless_validator::stateless_validator_inputs(
            bench_fixtures_dir.path(),
            ExecutionClient::Reth,
        )
        .unwrap();

        let len_inputs = inputs.len();
        assert_eq!(len_inputs, 1);

        run_guest(
            "stateless-validator/reth",
            &get_env_zkvm_or_default(vec![
                ErezkVM::SP1,
                ErezkVM::Risc0,
                ErezkVM::OpenVM,
                ErezkVM::Pico,
            ]),
            inputs,
            output_folder.path(),
            Action::Execute,
        );
        assert_executions_successful::<BlockMetadata>(output_folder.path(), len_inputs);
    }

    async fn empty_block(action: Action, el_zkvms: &[(ExecutionClient, ErezkVM)]) {
        for (el, zkvm) in el_zkvms {
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
            let inputs = stateless_validator::stateless_validator_inputs(
                bench_fixtures_dir.path(),
                el.clone(),
            )
            .unwrap();

            let len_inputs = inputs.len();
            assert_eq!(len_inputs, 1);

            run_guest(
                &format!("stateless-validator/{}", el.as_ref().to_lowercase()),
                &[*zkvm],
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
}
