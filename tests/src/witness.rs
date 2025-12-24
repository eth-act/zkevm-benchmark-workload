#[cfg(test)]
mod tests {
    use alloy_consensus::BlockHeader;
    use benchmark_runner::stateless_validator::read_benchmark_fixtures_folder;
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

        for fixture in fixtures {
            let parent_state_root = fixture
                .stateless_input
                .witness
                .headers
                .iter()
                .map(|h| alloy_rlp::decode_exact::<alloy_consensus::Header>(h).unwrap())
                .find(|h| h.number == fixture.stateless_input.block.number() - 1)
                .unwrap()
                .state_root;

            let account_addresses = fixture
                .stateless_input
                .witness
                .keys
                .iter()
                .filter(|k| k.len() == 20)
                .map(|k| alloy_primitives::Address::from_slice(k))
                .collect::<Vec<_>>();

            let ethereum_state = openvm_mpt::from_proof::from_execution_witness(
                parent_state_root,
                &fixture.stateless_input.witness.state,
                &account_addresses,
            )
            .unwrap();

            let ethereum_state_bytes = ethereum_state.encode_to_state_bytes();
            let bytes = bincode::serialize(&ethereum_state_bytes).unwrap();
            println!("ethereum state size: {}", bytes.len());
        }
    }
}
