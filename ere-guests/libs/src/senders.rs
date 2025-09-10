//! This module provies sender recovery helpers for witness generation and block stateless validation.

use alloy_consensus::BlockHeader;
use alloy_primitives::Address;
use k256::ecdsa::{VerifyingKey, signature::hazmat::PrehashVerifier};
use reth_chainspec::{ChainSpec, EthereumHardforks};
use reth_ethereum_primitives::{Block, TransactionSigned};
use reth_primitives_traits::{Block as _, RecoveredBlock};

/// Errors that can occur during stateless validation.
#[derive(Debug, thiserror::Error)]
pub enum StatelessValidationError {
    /// Error during signer recovery.
    #[error("signer recovery failed")]
    SignerRecovery,
    /// Error when signature has non-normalized s value in homestead block.
    #[error("signature s value not normalized for homestead block")]
    HomesteadSignatureNotNormalized,
    /// Custom error.
    #[error("{0}")]
    Custom(&'static str),
}

/// Verifies a transaction using its signature and the given public key.
///
/// Note: If the signature or the public key is incorrect, then this method
/// will return an error.
///
/// Returns the address derived from the public key.
fn recover_sender(
    vk: &VerifyingKey,
    tx: &TransactionSigned,
    is_homestead: bool,
) -> Result<Address, StatelessValidationError> {
    let sig = tx.signature();

    // non-normalized signatures are only valid pre-homestead
    let sig_is_normalized = sig.normalize_s().is_none();
    if is_homestead && !sig_is_normalized {
        return Err(StatelessValidationError::HomesteadSignatureNotNormalized);
    }

    sig.to_k256()
        .and_then(|sig| vk.verify_prehash(tx.signature_hash().as_slice(), &sig))
        .map_err(|_| StatelessValidationError::SignerRecovery)?;

    Ok(Address::from_public_key(vk))
}

/// Verifies all transactions in a block against a list of public keys and signatures.
///
/// Returns a `RecoveredBlock`
fn recover_block_with_public_keys<ChainSpec>(
    block: Block,
    public_keys: Vec<VerifyingKey>,
    chain_spec: &ChainSpec,
) -> Result<RecoveredBlock<Block>, StatelessValidationError>
where
    ChainSpec: EthereumHardforks,
{
    if block.body().transactions.len() != public_keys.len() {
        return Err(StatelessValidationError::Custom(
            "Number of public keys must match number of transactions",
        ));
    }

    // Determine if we're in the Homestead fork for signature validation
    let is_homestead = chain_spec.is_homestead_active_at_block(block.header().number());

    // Verify each transaction signature against its corresponding public key
    let senders = public_keys
        .iter()
        .zip(block.body().transactions())
        .map(|(vk, tx)| recover_sender(vk, tx, is_homestead))
        .collect::<Result<Vec<_>, _>>()?;

    // Create RecoveredBlock with verified senders
    let block_hash = block.hash_slow();
    Ok(RecoveredBlock::new(block, senders, block_hash))
}

/// Recover public keys from transaction signatures.
fn recover_signers<'a, I>(txs: I) -> Result<Vec<VerifyingKey>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = &'a TransactionSigned>,
{
    txs.into_iter()
        .enumerate()
        .map(|(i, tx)| {
            tx.signature()
                .recover_from_prehash(&tx.signature_hash())
                .map_err(|e| format!("failed to recover signature for tx #{i}: {e}").into())
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Recovers the block with verified senders from the given block and chain spec.
pub fn recover_block(
    block: Block,
    chain_spec: &ChainSpec,
) -> Result<RecoveredBlock<Block>, Box<dyn std::error::Error>>
where
    ChainSpec: EthereumHardforks,
{
    let public_keys = recover_signers(block.body().transactions.iter())?;
    let recovered_block = recover_block_with_public_keys(block, public_keys, &chain_spec)?;
    Ok(recovered_block)
}
