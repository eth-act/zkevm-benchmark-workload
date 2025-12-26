#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use benchmark_runner::stateless_validator::{
        read_benchmark_fixtures_folder, reth::get_input_full_validation,
    };
    use openvm_mpt::{
        from_proof::from_execution_witness, statelesstrie::OpenVMStatelessSparseTrie,
    };
    use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
    use sparsestate::SparseState;
    use std::{env, path::PathBuf};
    use tempfile::tempdir;

    use crate::utils::{untar, NoopPlatform};

    #[tokio::test(flavor = "multi_thread")]
    async fn transform_witness() {
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
            let mut input = get_input_full_validation(&fixture).unwrap();

            // Reth with Risc0 sparse MPT.
            RethStatelessValidatorGuest::<SparseState>::compute::<NoopPlatform>(input.clone());

            // Reth with OpenVM sparse MPT.
            {
                let pre_state_root = state_root_from_headers(
                    input.stateless_input.block.number - 1,
                    &input.stateless_input.witness.headers,
                );
                let tries_bytes =
                    from_execution_witness(pre_state_root, &input.stateless_input.witness)
                        .unwrap()
                        .encode_to_state_bytes();
                let bytes = bincode::serialize(&tries_bytes).unwrap();
                input.stateless_input.witness.state = vec![bytes.into()];
                RethStatelessValidatorGuest::<OpenVMStatelessSparseTrie>::compute::<NoopPlatform>(
                    input.clone(),
                );
            }
        }
    }

    fn state_root_from_headers(block_num: u64, headers: &[impl AsRef<[u8]>]) -> B256 {
        headers
            .iter()
            .find_map(|h| {
                let header = alloy_rlp::decode_exact::<alloy_consensus::Header>(h).unwrap();
                (header.number == block_num).then_some(header.state_root)
            })
            .unwrap()
    }
}
