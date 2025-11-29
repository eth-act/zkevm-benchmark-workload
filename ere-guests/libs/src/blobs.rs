//! Blob-related types and utilities for EIP-4844 blob transactions.

use anyhow::Result;
use c_kzg::Blob;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

/// Block body with optional DA proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBody {
    /// Encoded block body bytes.
    pub bytes: BlockBodyBytes,
    /// Optional DA proof for blob equivalence verification.
    pub da_proof: Option<BlockBodyDAProof>,
}

/// DA proof containing KZG commitments and proofs for blob equivalence verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBodyDAProof {
    /// KZG commitments for each blob.
    pub kzg_commitments: Vec<KzgCommitment>,
    /// KZG proofs for each blob.
    pub kzg_proofs: Vec<KzgProof>,
}

/// A 48-byte KZG commitment.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KzgCommitment(#[serde_as(as = "Bytes")] pub [u8; 48]);

impl From<KzgCommitment> for c_kzg::Bytes48 {
    fn from(c: KzgCommitment) -> Self {
        Self::new(c.0)
    }
}

/// A 48-byte KZG proof.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KzgProof(#[serde_as(as = "Bytes")] pub [u8; 48]);

impl From<KzgProof> for c_kzg::Bytes48 {
    fn from(p: KzgProof) -> Self {
        Self::new(p.0)
    }
}

/// Encoded block body bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlockBodyBytes {
    /// Raw uncompressed bytes.
    Raw(Vec<u8>),
    /// Snappy-compressed bytes.
    CompressedSnappy(Vec<u8>),
}

impl BlockBodyBytes {
    /// Returns the block body bytes as a slice.
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::CompressedSnappy(bytes) | Self::Raw(bytes) => bytes,
        }
    }
}

/// Block body DA options.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockBodyEncoding {
    /// Enable DA with raw body encoding.
    Raw,
    /// Enable DA with Snappy-compressed body encoding.
    Snappy,
}

/// Returns the block body with the specified encoding and optional proof of equivalence.
pub fn get_block_body(
    serialized: Vec<u8>,
    block_body_encoding: BlockBodyEncoding,
    with_proof_of_equivalence: bool,
) -> Result<BlockBody> {
    let block_body_bytes = match block_body_encoding {
        BlockBodyEncoding::Raw => BlockBodyBytes::Raw(serialized),
        BlockBodyEncoding::Snappy => {
            let compressed = snap::raw::Encoder::new()
                .compress_vec(&serialized)
                .map_err(|e| anyhow::anyhow!("compressing block body with snappy: {e}"))?;
            BlockBodyBytes::CompressedSnappy(compressed)
        }
    };

    let da_proof = with_proof_of_equivalence
        .then(|| gen_da_proof(block_body_bytes.as_slice()))
        .transpose()?;

    Ok(BlockBody {
        bytes: block_body_bytes,
        da_proof,
    })
}

fn gen_da_proof(bytes: &[u8]) -> Result<BlockBodyDAProof> {
    let blobs = partition_into_blobs(bytes);
    let mut kzg_commitments = Vec::with_capacity(blobs.len());
    let mut kzg_proofs = Vec::with_capacity(blobs.len());

    let kzg_settings = c_kzg::ethereum_kzg_settings(8);
    for blob in blobs {
        let commitment = kzg_settings.blob_to_kzg_commitment(&blob)?;
        let proof = kzg_settings.compute_blob_kzg_proof(&blob, &commitment.to_bytes())?;
        kzg_commitments.push(KzgCommitment(commitment.to_bytes().into_inner()));
        kzg_proofs.push(KzgProof(proof.to_bytes().into_inner()));
    }

    Ok(BlockBodyDAProof {
        kzg_commitments,
        kzg_proofs,
    })
}

const BYTES_PER_FIELD_ELEMENT: usize = 32;
const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
const BYTES_PER_BLOB: usize = FIELD_ELEMENTS_PER_BLOB * BYTES_PER_FIELD_ELEMENT;
const USABLE_BYTES_PER_ELEMENT: usize = BYTES_PER_FIELD_ELEMENT - 1; // Leave high byte as 0 to stay below modulus
const USABLE_BYTES_PER_BLOB: usize = FIELD_ELEMENTS_PER_BLOB * USABLE_BYTES_PER_ELEMENT;

/// Partitions data into EIP-4844 blobs.
pub fn partition_into_blobs(data: &[u8]) -> Vec<Blob> {
    if data.is_empty() {
        return Vec::new();
    }

    let num_blobs = data.len().div_ceil(USABLE_BYTES_PER_BLOB);
    let mut blobs = Vec::with_capacity(num_blobs);
    let mut offset = 0;

    for _ in 0..num_blobs {
        let mut blob_data = [0u8; BYTES_PER_BLOB];

        for fe_idx in 0..FIELD_ELEMENTS_PER_BLOB {
            if offset >= data.len() {
                break;
            }
            let chunk_size = (data.len() - offset).min(USABLE_BYTES_PER_ELEMENT);
            let blob_offset = fe_idx * BYTES_PER_FIELD_ELEMENT + 1; // +1 leaves high byte as 0
            blob_data[blob_offset..blob_offset + chunk_size]
                .copy_from_slice(&data[offset..offset + chunk_size]);
            offset += chunk_size;
        }

        blobs.push(c_kzg::Blob::new(blob_data));
    }

    blobs
}
