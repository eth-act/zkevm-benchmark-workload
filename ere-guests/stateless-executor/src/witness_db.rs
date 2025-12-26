//! Provides the [`WitnessDatabase`] type for EVM execution backed by witness data.

use alloc::collections::btree_map::BTreeMap;
use alloy_primitives::{map::B256Map, Address, B256, U256};
use reth_errors::ProviderError;
use reth_revm::{bytecode::Bytecode, state::AccountInfo, Database};
use reth_stateless::trie::StatelessTrie;

/// An EVM database implementation backed by witness data.
///
/// This struct implements the [`reth_revm::Database`] trait, allowing the EVM to execute
/// transactions using state from a [`StatelessTrie`] implementation.
#[derive(Debug)]
pub struct WitnessDatabase<'a, T>
where
    T: StatelessTrie,
{
    /// Map of block numbers to block hashes (for BLOCKHASH opcode).
    block_hashes_by_block_number: BTreeMap<u64, B256>,
    /// Map of code hashes to bytecode.
    bytecode: B256Map<Bytecode>,
    /// The sparse state trie containing account and storage state.
    trie: &'a T,
}

impl<'a, T> WitnessDatabase<'a, T>
where
    T: StatelessTrie,
{
    /// Creates a new [`WitnessDatabase`] instance.
    pub const fn new(
        trie: &'a T,
        bytecode: B256Map<Bytecode>,
        ancestor_hashes: BTreeMap<u64, B256>,
    ) -> Self {
        Self {
            trie,
            block_hashes_by_block_number: ancestor_hashes,
            bytecode,
        }
    }
}

impl<T> Database for WitnessDatabase<'_, T>
where
    T: StatelessTrie,
{
    type Error = ProviderError;

    /// Get basic account information.
    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        self.trie.account(address).map(|opt| {
            opt.map(|account| AccountInfo {
                balance: account.balance,
                nonce: account.nonce,
                code_hash: account.code_hash,
                code: None,
            })
        })
    }

    /// Get storage value of an account at a specific slot.
    fn storage(&mut self, address: Address, slot: U256) -> Result<U256, Self::Error> {
        self.trie.storage(address, slot)
    }

    /// Get account code by its hash.
    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        self.bytecode.get(&code_hash).cloned().ok_or_else(|| {
            ProviderError::TrieWitnessError(alloc::format!("bytecode for {code_hash} not found"))
        })
    }

    /// Get block hash by block number.
    fn block_hash(&mut self, block_number: u64) -> Result<B256, Self::Error> {
        self.block_hashes_by_block_number
            .get(&block_number)
            .copied()
            .ok_or(ProviderError::StateForNumberNotFound(block_number))
    }
}
