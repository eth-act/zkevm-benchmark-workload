#[cfg(test)]
mod tests {
    use alloy_primitives::{map::B256Map, B256};
    use benchmark_runner::stateless_validator::{
        read_benchmark_fixtures_folder, reth::get_input_full_validation,
    };
    use openvm_mpt::{
        from_proof::from_execution_witness, statelesstrie::OpenVMStatelessSparseTrie,
    };
    use reth_chainspec::ChainSpec;
    use reth_errors::ProviderError;
    use reth_evm_ethereum::EthEvmConfig;
    use reth_stateless::{
        trie::StatelessSparseTrie, validation::StatelessValidationError, Genesis,
    };
    use reth_trie_common::HashedPostState;
    use revm_bytecode::Bytecode;
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
            let block_num = fixture.stateless_input.block.number;
            let mut input = get_input_full_validation(&fixture).unwrap();

            let genesis = Genesis {
                config: input.stateless_input.chain_config.clone(),
                ..Default::default()
            };
            let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
            let evm_config = EthEvmConfig::new(chain_spec.clone());

            // Run normal Reth.
            reth_stateless::stateless_validation_with_trie::<StatelessSparseTrie, _, _>(
                input.stateless_input.block.clone(),
                input.public_keys.clone(),
                input.stateless_input.witness.clone(),
                chain_spec.clone(),
                evm_config.clone(),
            )
            .unwrap();

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
            let res =
                reth_stateless::stateless_validation_with_trie::<OpenVMStatelessSparseTrie, _, _>(
                    input.stateless_input.block,
                    input.public_keys,
                    input.stateless_input.witness,
                    chain_spec,
                    evm_config,
                );
            match res {
                Ok(_) => println!(
                    "Stateless validation succeeded for block num {}.",
                    block_num
                ),
                Err(e) => panic!(
                    "Stateless validation failed for block num {}: {:?}",
                    block_num, e
                ),
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

    #[derive(Debug)]
    struct Wrapper<T: reth_stateless::StatelessTrie> {
        inner: T,
    }

    impl<T: reth_stateless::StatelessTrie> reth_stateless::StatelessTrie for Wrapper<T> {
        fn new(
            witness: &reth_stateless::ExecutionWitness,
            pre_state_root: B256,
        ) -> Result<(Self, B256Map<Bytecode>), StatelessValidationError>
        where
            Self: Sized,
        {
            println!(
                "StatelessTrie::new called with pre_state_root: {:?}",
                pre_state_root
            );
            // Note: We can't delegate `new` to `internal` since `internal` is already constructed.
            // This method would be used to create a new Wrapper from scratch.
            // For now, we'll use the default StatelessSparseTrie implementation.
            let (inner, bytecodes) = T::new(witness, pre_state_root)?;
            let wrapper = Self { inner };
            println!(
                "StatelessTrie::new returning with {} bytecodes",
                bytecodes.len()
            );
            Ok((wrapper, bytecodes))
        }

        fn account(
            &self,
            address: alloy_primitives::Address,
        ) -> Result<Option<reth_trie_common::TrieAccount>, ProviderError> {
            println!("StatelessTrie::account called with address: {:?}", address);
            let result = self.inner.account(address);
            println!("StatelessTrie::account returning: {:?}", result);
            result
        }

        fn storage(
            &self,
            address: alloy_primitives::Address,
            slot: alloy_primitives::U256,
        ) -> Result<alloy_primitives::U256, ProviderError> {
            println!(
                "StatelessTrie::storage called with address: {:?}, slot: {:?}",
                address, slot
            );
            let result = self.inner.storage(address, slot);
            println!("StatelessTrie::storage returning: {:?}", result);
            result
        }

        fn calculate_state_root(
            &mut self,
            state: HashedPostState,
        ) -> Result<B256, StatelessValidationError> {
            println!(
                "StatelessTrie::calculate_state_root called with {} accounts, {} storages",
                state.accounts.len(),
                state.storages.len()
            );
            let result = self.inner.calculate_state_root(state);
            println!(
                "StatelessTrie::calculate_state_root returning: {:?}",
                result
            );
            result
        }
    }
}
