#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use benchmark_runner::stateless_validator::read_benchmark_fixtures_folder;
    use openvm_mpt::from_proof::from_execution_witness;
    use std::{env, path::PathBuf};
    use tempfile::tempdir;

    use crate::utils::untar;

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

        for mut fixture in fixtures {
            let pre_state_root = state_root_from_headers(
                fixture.stateless_input.block.number - 1,
                &fixture.stateless_input.witness.headers,
            );
            let tries_bytes =
                from_execution_witness(pre_state_root, &fixture.stateless_input.witness)
                    .unwrap()
                    .encode_to_state_bytes();
            let bytes = bincode::serialize(&tries_bytes).unwrap();
            fixture.stateless_input.witness.state = vec![bytes.into()];
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
