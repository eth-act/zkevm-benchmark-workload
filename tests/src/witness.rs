#[cfg(test)]
mod tests {
    use benchmark_runner::stateless_validator::{
        read_benchmark_fixtures_folder,
        reth::{get_input_full_validation, transform_witness},
    };
    use openvm_mpt::statelesstrie::OpenVMStatelessSparseTrie;
    use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
    use sparsestate::SparseState;
    use std::{env, path::PathBuf};
    use tempfile::tempdir;

    use crate::utils::{untar, NoopPlatform};

    #[tokio::test(flavor = "multi_thread")]
    async fn sparse_mpts() {
        println!("Starting transform_witness test...");
        let bench_fixtures_dir = tempdir().unwrap();
        untar(
            &PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("assets/mainnet-zkevm-fixtures-input.tar.gz"),
            bench_fixtures_dir.path(),
        );
        let input_folder = &bench_fixtures_dir
            .path()
            .join("mainnet-zkevm-fixtures-input");

        let fixtures = read_benchmark_fixtures_folder(input_folder).unwrap();

        for fixture in fixtures {
            println!("Processing fixture: {}", fixture.name);
            let input = get_input_full_validation(&fixture).unwrap();

            // Reth with Risc0 sparse MPT.
            RethStatelessValidatorGuest::<SparseState>::compute::<NoopPlatform>(input.clone());

            // Reth with OpenVM sparse MPT.
            {
                let input = transform_witness(input);

                RethStatelessValidatorGuest::<OpenVMStatelessSparseTrie>::compute::<NoopPlatform>(
                    input,
                );
            }
        }
    }
}
