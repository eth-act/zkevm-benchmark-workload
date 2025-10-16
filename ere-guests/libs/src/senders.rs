//! This module provies sender recovery helpers for witness generation and block stateless validation.

use reth_ethereum_primitives::TransactionSigned;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

/// Uncompressed public key wrapper that supports bincode serialization
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncompressedPublicKey(#[serde_as(as = "Bytes")] pub [u8; 65]);

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

/// Recover public keys from transaction signatures.
pub fn recover_signers<'a, I>(
    txs: I,
) -> Result<Vec<UncompressedPublicKey>, Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = &'a TransactionSigned>,
{
    txs.into_iter()
        .enumerate()
        .map(|(i, tx)| {
            tx.signature()
                .recover_from_prehash(&tx.signature_hash())
                .map(|keys| {
                    UncompressedPublicKey(
                        TryInto::<[u8; 65]>::try_into(keys.to_encoded_point(false).as_bytes())
                            .unwrap(),
                    )
                })
                .map_err(|e| format!("failed to recover signature for tx #{i}: {e}").into())
        })
        .collect::<Result<Vec<UncompressedPublicKey>, _>>()
}
