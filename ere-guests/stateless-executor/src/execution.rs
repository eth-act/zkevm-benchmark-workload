//! Core execution function for stateless block execution without validation.

use crate::witness_db::WitnessDatabase;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use alloy_consensus::{BlockHeader, Header};
use alloy_primitives::{keccak256, Address, B256};
use reth_chainspec::{EthChainSpec, EthereumHardforks};
use reth_ethereum_primitives::{Block, EthPrimitives, TransactionSigned};
use reth_evm::{execute::Executor, ConfigureEvm};
use reth_primitives_traits::{Block as _, RecoveredBlock, SealedHeader};
use reth_stateless::{trie::StatelessTrie, UncompressedPublicKey};

/// Errors that can occur during stateless execution.
#[derive(Debug, thiserror::Error)]
pub enum StatelessExecutionError {
    /// Error during stateless block execution.
    #[error("stateless block execution failed: {0}")]
    ExecutionFailed(String),

    /// Error during signer recovery.
    #[error("signer recovery failed")]
    SignerRecovery,

    /// Error when signature has non-normalized s value in homestead block.
    #[error("signature s value not normalized for homestead block")]
    HomesteadSignatureNotNormalized,

    /// Error when the number of public keys doesn't match transactions.
    #[error("number of public keys ({keys}) must match number of transactions ({txs})")]
    PublicKeyCountMismatch {
        /// Number of public keys provided.
        keys: usize,
        /// Number of transactions in the block.
        txs: usize,
    },

    /// Error when building state from witness.
    #[error("failed to build state from witness")]
    WitnessBuildFailed,

    /// Error deserializing ancestor headers.
    #[error("failed to deserialize ancestor headers")]
    HeaderDeserializationFailed,

    /// Error when no ancestor headers provided (needed for state root).
    #[error("missing ancestor headers")]
    MissingAncestorHeader,
}

/// Performs stateless execution of a block without any validation.
///
/// This function executes all transactions in a block using the EVM, but skips:
/// - Pre-execution consensus validation (header checks, ancestor verification)
/// - Post-execution consensus checks (receipts root, gas used validation)
/// - State root verification (both pre-state and post-state)
///
/// This is useful for benchmarking pure EVM execution cycles in zkVMs.
///
/// # Type Parameters
///
/// - `T`: The `StatelessTrie` implementation to use (e.g., `SparseState`)
/// - `ChainSpec`: The chain specification
/// - `E`: The EVM configuration
///
/// # Returns
///
/// Returns `true` if EVM execution succeeded for all transactions, `false` otherwise.
pub fn stateless_execution_with_trie<T, ChainSpec, E>(
    current_block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    witness: reth_stateless::ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    evm_config: E,
) -> bool
where
    T: StatelessTrie,
    ChainSpec: Send + Sync + EthChainSpec<Header = Header> + EthereumHardforks + core::fmt::Debug,
    E: ConfigureEvm<Primitives = EthPrimitives> + Clone + 'static,
{
    match stateless_execution_inner::<T, ChainSpec, E>(
        current_block,
        public_keys,
        witness,
        chain_spec,
        evm_config,
    ) {
        Ok(()) => true,
        Err(_) => false,
    }
}

/// Inner implementation that returns Result for error handling.
fn stateless_execution_inner<T, ChainSpec, E>(
    current_block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    witness: reth_stateless::ExecutionWitness,
    chain_spec: Arc<ChainSpec>,
    evm_config: E,
) -> Result<(), StatelessExecutionError>
where
    T: StatelessTrie,
    ChainSpec: Send + Sync + EthChainSpec<Header = Header> + EthereumHardforks + core::fmt::Debug,
    E: ConfigureEvm<Primitives = EthPrimitives> + Clone + 'static,
{
    // Step 1: Recover block with public keys (we still need signers for execution)
    let current_block = recover_block_with_public_keys(current_block, public_keys, &*chain_spec)?;

    // Step 2: Build ancestor hashes from witness headers (needed for BLOCKHASH opcode)
    // We parse headers but skip contiguity/limit validation
    let (ancestor_hashes, parent_state_root) =
        build_ancestor_hashes_unchecked(&current_block, &witness)?;

    // Step 3: Build state from witness using the StatelessTrie trait
    // Note: We pass the parent state root but don't validate it matches
    let (trie, bytecode) =
        T::new(&witness, parent_state_root).map_err(|_| StatelessExecutionError::WitnessBuildFailed)?;

    // Step 4: Create an in-memory database for EVM execution
    let db = WitnessDatabase::new(&trie, bytecode, ancestor_hashes);

    // Step 5: Execute the block (this is the core EVM execution)
    let executor = evm_config.executor(db);
    let _output = executor
        .execute(&current_block)
        .map_err(|e| StatelessExecutionError::ExecutionFailed(e.to_string()))?;

    // SKIP: Post-execution validation (validate_block_post_execution)
    // SKIP: State root computation and verification

    Ok(())
}

/// Builds ancestor hashes from witness headers without validation.
///
/// Returns the ancestor hashes map and the parent's state root.
fn build_ancestor_hashes_unchecked(
    current_block: &RecoveredBlock<Block>,
    witness: &reth_stateless::ExecutionWitness,
) -> Result<(BTreeMap<u64, B256>, B256), StatelessExecutionError> {
    let mut ancestor_headers: Vec<_> = witness
        .headers
        .iter()
        .map(|bytes| {
            let hash = keccak256(bytes);
            alloy_rlp::decode_exact::<Header>(bytes)
                .map(|h| SealedHeader::new(h, hash))
                .map_err(|_| StatelessExecutionError::HeaderDeserializationFailed)
        })
        .collect::<Result<_, _>>()?;

    // Sort headers by block number (ascending)
    ancestor_headers.sort_by_key(|header| header.number());

    // Get the parent header (last in sorted list = highest block number = parent)
    let parent = ancestor_headers
        .last()
        .ok_or(StatelessExecutionError::MissingAncestorHeader)?;
    let parent_state_root = parent.state_root;

    let mut ancestor_hashes = BTreeMap::new();
    let mut child_header = current_block.sealed_header();

    // Build the hash map (skip contiguity validation for execution-only mode)
    for parent_header in ancestor_headers.iter().rev() {
        let parent_hash = child_header.parent_hash();
        ancestor_hashes.insert(parent_header.number, parent_hash);
        child_header = parent_header;
    }

    Ok((ancestor_hashes, parent_state_root))
}

/// Verifies all transactions in a block against a list of public keys.
///
/// Returns a `RecoveredBlock` with verified senders.
fn recover_block_with_public_keys<ChainSpec>(
    block: Block,
    public_keys: Vec<UncompressedPublicKey>,
    chain_spec: &ChainSpec,
) -> Result<RecoveredBlock<Block>, StatelessExecutionError>
where
    ChainSpec: EthereumHardforks,
{
    if block.body().transactions.len() != public_keys.len() {
        return Err(StatelessExecutionError::PublicKeyCountMismatch {
            keys: public_keys.len(),
            txs: block.body().transactions.len(),
        });
    }

    // Determine if we're in the Homestead fork for signature validation
    let is_homestead = chain_spec.is_homestead_active_at_block(block.header().number());

    // Verify each transaction signature against its corresponding public key
    let senders = public_keys
        .iter()
        .zip(block.body().transactions())
        .map(|(vk, tx)| verify_and_compute_sender(vk, tx, is_homestead))
        .collect::<Result<Vec<_>, _>>()?;

    // Create RecoveredBlock with verified senders
    let block_hash = block.hash_slow();
    Ok(RecoveredBlock::new(block, senders, block_hash))
}

/// Verifies a transaction using its signature and the given public key.
fn verify_and_compute_sender(
    vk: &UncompressedPublicKey,
    tx: &TransactionSigned,
    is_homestead: bool,
) -> Result<Address, StatelessExecutionError> {
    use k256::ecdsa::{signature::hazmat::PrehashVerifier, VerifyingKey};

    let sig = tx.signature();

    // Non-normalized signatures are only valid pre-homestead
    let sig_is_normalized = sig.normalize_s().is_none();
    if is_homestead && !sig_is_normalized {
        return Err(StatelessExecutionError::HomesteadSignatureNotNormalized);
    }

    let sig_hash = tx.signature_hash();

    let vk =
        VerifyingKey::from_sec1_bytes(vk).map_err(|_| StatelessExecutionError::SignerRecovery)?;

    sig.to_k256()
        .and_then(|sig| vk.verify_prehash(sig_hash.as_slice(), &sig))
        .map_err(|_| StatelessExecutionError::SignerRecovery)?;

    Ok(Address::from_public_key(&vk))
}
