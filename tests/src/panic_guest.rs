#[cfg(test)]
mod tests {
    use crate::utils::{
        assert_executions_crashed, assert_proving_crashed, get_env_zkvm_or_default, run_guest,
    };

    use benchmark_runner::{empty_program, runner::Action};
    use ere_dockerized::ErezkVM;
    use tempfile::tempdir;

    #[tokio::test]
    async fn execute_panic_program() {
        let zkvms = get_env_zkvm_or_default(vec![ErezkVM::SP1, ErezkVM::Risc0, ErezkVM::Zisk]);
        panic_program(&zkvms, Action::Execute).await;
    }

    #[tokio::test]
    async fn prove_panic_program() {
        let zkvms = get_env_zkvm_or_default(vec![ErezkVM::SP1, ErezkVM::Risc0]);
        panic_program(&zkvms, Action::Prove).await;
    }

    async fn panic_program(zkvms: &[ErezkVM], action: Action) {
        let output_folder = tempdir().unwrap();
        let input = empty_program::empty_program_input();
        run_guest(
            "panic-guest",
            zkvms,
            vec![input],
            output_folder.path(),
            action,
        );
        match action {
            Action::Prove => assert_proving_crashed::<()>(output_folder.path(), 1),
            Action::Execute => assert_executions_crashed::<()>(output_folder.path(), 1),
        }
    }
}
