#[cfg(test)]
mod tests {
    use crate::utils::{
        assert_executions_successful, assert_proving_successful, get_env_zkvm_or_default, run_guest,
    };

    use benchmark_runner::{empty_program, runner::Action};
    use ere_dockerized::zkVMKind;
    use tempfile::tempdir;

    #[tokio::test(flavor = "multi_thread")]
    async fn execute_empty_program() {
        let zkvms = get_env_zkvm_or_default(vec![
            zkVMKind::SP1,
            zkVMKind::Risc0,
            zkVMKind::OpenVM,
            zkVMKind::Zisk,
            zkVMKind::Pico,
        ]);
        empty_program(&zkvms, Action::Execute).await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn prove_empty_program() {
        let zkvms = get_env_zkvm_or_default(vec![
            zkVMKind::SP1,
            zkVMKind::Risc0,
            zkVMKind::OpenVM,
            zkVMKind::Pico,
        ]);
        empty_program(&zkvms, Action::Prove).await;
    }

    async fn empty_program(zkvms: &[zkVMKind], action: Action) {
        let output_folder = tempdir().unwrap();
        let input = empty_program::empty_program_input().unwrap();
        run_guest(
            "empty-program",
            zkvms,
            vec![input],
            output_folder.path(),
            None,
            action,
        );
        match action {
            Action::Prove => assert_proving_successful::<()>(output_folder.path(), 1),
            Action::Execute => assert_executions_successful::<()>(output_folder.path(), 1),
        }
    }
}
