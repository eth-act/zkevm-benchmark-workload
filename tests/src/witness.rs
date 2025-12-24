#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use benchmark_runner::stateless_validator::{
        read_benchmark_fixtures_folder, reth::get_input_full_validation,
    };
    use openvm_mpt::{
        from_proof::from_execution_witness, statelesstrie::OpenVMStatelessSparseTrie,
    };
    use reth_chainspec::ChainSpec;
    use reth_evm_ethereum::EthEvmConfig;
    use reth_stateless::Genesis;
    use std::{env, path::PathBuf, sync::Arc};
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
            let input = get_input_full_validation(&fixture).unwrap();

            let genesis = Genesis {
                config: input.stateless_input.chain_config.clone(),
                ..Default::default()
            };
            let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
            let evm_config = EthEvmConfig::new(chain_spec.clone());

            // Run normal Reth.
            reth_stateless::stateless_validation(
                input.stateless_input.block.clone(),
                input.public_keys.clone(),
                input.stateless_input.witness.clone(),
                chain_spec.clone(),
                evm_config.clone(),
            )
            .unwrap();

            let mut stateless_input = fixture.stateless_input;
            let pre_state_root = state_root_from_headers(
                stateless_input.block.number - 1,
                &stateless_input.witness.headers,
            );
            let tries_bytes = from_execution_witness(pre_state_root, &stateless_input.witness)
                .unwrap()
                .encode_to_state_bytes();
            let bytes = bincode::serialize(&tries_bytes).unwrap();
            stateless_input.witness.state = vec![bytes.into()];
            reth_stateless::stateless_validation_with_trie::<OpenVMStatelessSparseTrie, _, _>(
                input.stateless_input.block,
                input.public_keys,
                input.stateless_input.witness,
                chain_spec,
                evm_config,
            )
            .unwrap();
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
