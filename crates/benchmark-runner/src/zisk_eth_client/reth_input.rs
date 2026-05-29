//! Vendored encoding of the external zisk-eth-client guest input.
//!
//! Reproduces `guest_reth::RethInput` byte-for-byte so a benchmark fixture can
//! be converted into the guest's stdin in process. The guest reads a single
//! `RethInput` record (bincode standard) bundling the stateless input with the
//! host-recovered transaction signer public keys, and commits the validated
//! block hash. The `block`/`chain_config` fields carry the same
//! `serde_bincode_compat` adapters the guest uses, so the produced bytes match
//! exactly. Types resolve against the `-v1` aliased reth/alloy crates.

use alloy_genesis_v1::ChainConfig;
use alloy_primitives_v1::B256;
use alloy_rpc_types_debug_v1::ExecutionWitness;
use anyhow::{anyhow, Context, Result};
use rayon::prelude::*;
use reth_ethereum_primitives_v1::{Block, TransactionSigned};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

/// 65-byte uncompressed secp256k1 public key, matching the guest's newtype
/// (serialized as raw bytes via the `Bytes` adapter, like the guest).
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UncompressedPublicKey(#[serde_as(as = "Bytes")] [u8; 65]);

/// Mirrors the patched `stateless_reth::StatelessInput` bincode wire format.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatelessInput {
    #[serde_as(
        as = "reth_primitives_traits_v1::serde_bincode_compat::Block<reth_ethereum_primitives_v1::TransactionSigned, alloy_consensus_v1::Header>"
    )]
    block: Block,
    witness: ExecutionWitness,
    #[serde_as(as = "alloy_genesis_v1::serde_bincode_compat::ChainConfig<'_>")]
    chain_config: ChainConfig,
}

/// Mirrors `guest_reth::RethInput`.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RethInput {
    stateless_input: StatelessInput,
    public_keys: Vec<UncompressedPublicKey>,
}

/// On-disk benchmark fixture shape, with `block` in native reth JSON (the form
/// the upstream workload writes), distinct from the bincode-compat form above.
#[serde_as]
#[derive(Deserialize)]
struct RawFixture {
    name: String,
    stateless_input: RawStatelessInput,
}

#[serde_as]
#[derive(Deserialize)]
struct RawStatelessInput {
    block: Block,
    witness: ExecutionWitness,
    #[serde_as(as = "alloy_genesis_v1::serde_bincode_compat::ChainConfig<'_>")]
    chain_config: ChainConfig,
}

/// Result of converting a benchmark fixture into guest input.
pub(crate) struct ConvertedFixture {
    /// Fixture name.
    pub(crate) name: String,
    /// Gas used by the block, recorded in the metrics metadata.
    pub(crate) gas_used: u64,
    /// Guest stdin (bincode-encoded `RethInput`).
    pub(crate) stdin: Vec<u8>,
    /// Expected committed output, the block hash encoded as the guest commits it.
    pub(crate) expected: Vec<u8>,
}

/// Converts a benchmark fixture JSON into the guest stdin and expected output.
///
/// The expected output is the block hash encoded exactly as the guest commits
/// it via `ziskos::io::commit`.
pub(crate) fn convert_fixture_json(json: &[u8]) -> Result<ConvertedFixture> {
    let fixture: RawFixture =
        serde_json::from_slice(json).context("Failed to parse stateless validation fixture")?;

    let public_keys = recover_signers(&fixture.stateless_input.block.body.transactions)
        .context("Failed to recover transaction signer public keys")?;

    let gas_used = fixture.stateless_input.block.header.gas_used;
    let block_hash: B256 = fixture.stateless_input.block.header.hash_slow();
    let expected = bincode::serde::encode_to_vec(block_hash, bincode::config::standard())
        .context("Failed to encode expected block hash")?;

    let reth_input = RethInput {
        stateless_input: StatelessInput {
            block: fixture.stateless_input.block,
            witness: fixture.stateless_input.witness,
            chain_config: fixture.stateless_input.chain_config,
        },
        public_keys,
    };
    let stdin = bincode::serde::encode_to_vec(&reth_input, bincode::config::standard())
        .context("Failed to serialize RethInput")?;

    Ok(ConvertedFixture {
        name: fixture.name,
        gas_used,
        stdin,
        expected,
    })
}

/// Recovers the uncompressed signer public key for each transaction.
fn recover_signers(txs: &[TransactionSigned]) -> Result<Vec<UncompressedPublicKey>> {
    txs.par_iter()
        .enumerate()
        .map(|(i, tx)| {
            let key = tx
                .signature()
                .recover_from_prehash(&tx.signature_hash())
                .with_context(|| format!("failed to recover signature for tx #{i}"))?;
            let encoded: [u8; 65] = key
                .to_encoded_point(false)
                .as_bytes()
                .try_into()
                .map_err(|e| anyhow!("failed to encode public key for tx #{i}: {e}"))?;
            Ok(UncompressedPublicKey(encoded))
        })
        .collect()
}
