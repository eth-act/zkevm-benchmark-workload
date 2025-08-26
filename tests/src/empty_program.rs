#[cfg(test)]
mod tests {
    use crate::utils::{assert_executions_successful, assert_proving_successful, run_guest};

    use benchmark_runner::{guest_programs, Action};
    use ere_dockerized::ErezkVM;
    use tempfile::tempdir;

    #[tokio::test]
    async fn execute_empty_program() {
        empty_program(
            &[ErezkVM::SP1, ErezkVM::Risc0, ErezkVM::Zisk],
            Action::Execute,
        )
        .await;
    }

    #[tokio::test]
    async fn prove_empty_program() {
        let zkvms = vec![ErezkVM::SP1, ErezkVM::Risc0];
        empty_program(&zkvms, Action::Prove).await;
    }

    async fn empty_program(zkvms: &[ErezkVM], action: Action) {
        let output_folder = tempdir().unwrap();
        let input = guest_programs::empty_program_input();
        run_guest(
            "empty-program",
            zkvms,
            vec![input],
            output_folder.path(),
            action,
        );
        match action {
            Action::Prove => assert_proving_successful::<()>(output_folder.path()),
            Action::Execute => assert_executions_successful::<()>(output_folder.path()),
        }
    }
}
