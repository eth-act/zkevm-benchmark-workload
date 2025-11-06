//! OpenVM Crypto Implementation for REVM
//!
//! This module provides OpenVM-optimized implementations of cryptographic operations
//! for both transaction validation (via Alloy crypto provider) and precompile execution.

use alloy_consensus::crypto::{
    backend::{install_default_provider, CryptoProvider},
    RecoveryError,
};
use alloy_primitives::Address;
use openvm_ecc_guest::{
    algebra::IntMod,
    weierstrass::{IntrinsicCurve, WeierstrassPoint},
    AffinePoint,
};
use openvm_k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use openvm_keccak256::keccak256;
use openvm_kzg::{Bytes32, Bytes48, KzgProof};
#[allow(unused_imports, clippy::single_component_path_imports)]
use openvm_p256; // ensure this is linked in for the standard OpenVM config
use openvm_pairing::{
    bn254::{Bn254, Fp, Fp2, G1Affine, G2Affine, Scalar},
    PairingCheck,
};
use revm::{
    install_crypto,
    precompile::{Crypto, PrecompileError},
};
use std::{sync::Arc, vec::Vec};

// BN254 constants
const FQ_LEN: usize = 32;
const G1_LEN: usize = 64;
const G2_LEN: usize = 128;
/// SCALAR_LEN specifies the number of bytes needed to represent an Fr element.
/// This is an element in the scalar field of BN254.
const SCALAR_LEN: usize = 32;

/// OpenVM k256 backend for Alloy crypto operations (transaction validation)
#[derive(Debug, Default)]
struct OpenVmK256Provider;

impl CryptoProvider for OpenVmK256Provider {
    fn recover_signer_unchecked(
        &self,
        sig: &[u8; 65],
        msg: &[u8; 32],
    ) -> Result<Address, RecoveryError> {
        // Extract components: sig[0..32]=r, sig[32..64]=s, sig[64]=recovery_id
        // Parse signature using OpenVM k256
        let mut signature = Signature::from_slice(&sig[..64]).map_err(|_| RecoveryError::new())?;

        // Normalize signature if needed
        let mut recid = sig[64];
        if let Some(sig_normalized) = signature.normalize_s() {
            signature = sig_normalized;
            recid ^= 1;
        }

        // Create recovery ID
        let recovery_id = RecoveryId::from_byte(recid).ok_or(RecoveryError::new())?;

        // Recover public key using OpenVM
        let recovered_key =
            VerifyingKey::recover_from_prehash_noverify(msg, &signature.to_bytes(), recovery_id)
                .map_err(|_| RecoveryError::new())?;

        // Get public key coordinates
        let public_key = recovered_key.as_affine();
        let mut encoded_pubkey = [0u8; 64];
        encoded_pubkey[..32].copy_from_slice(&WeierstrassPoint::x(public_key).to_be_bytes());
        encoded_pubkey[32..].copy_from_slice(&WeierstrassPoint::y(public_key).to_be_bytes());

        // Hash to get Ethereum address
        let pubkey_hash = keccak256(&encoded_pubkey);
        let address_bytes = &pubkey_hash[12..32]; // Last 20 bytes

        Ok(Address::from_slice(address_bytes))
    }
}

/// OpenVM custom crypto implementation for faster precompiles
#[derive(Debug, Default)]
struct OpenVmCrypto;

impl Crypto for OpenVmCrypto {
    /// Custom SHA-256 implementation with openvm optimization
    fn sha256(&self, input: &[u8]) -> [u8; 32] {
        openvm_sha2::sha256(input)
    }

    /// Custom BN254 G1 addition with openvm optimization
    fn bn254_g1_add(&self, p1_bytes: &[u8], p2_bytes: &[u8]) -> Result<[u8; 64], PrecompileError> {
        let p1 = read_g1_point(p1_bytes)?;
        let p2 = read_g1_point(p2_bytes)?;
        let result = p1 + p2;
        Ok(encode_g1_point(result))
    }

    /// Custom BN254 G1 scalar multiplication with openvm optimization
    fn bn254_g1_mul(
        &self,
        point_bytes: &[u8],
        scalar_bytes: &[u8],
    ) -> Result<[u8; 64], PrecompileError> {
        let p = read_g1_point(point_bytes)?;
        let s = read_scalar(scalar_bytes);
        let result = Bn254::msm(&[s], &[p]);
        Ok(encode_g1_point(result))
    }

    /// Custom BN254 pairing check with openvm optimization
    fn bn254_pairing_check(&self, pairs: &[(&[u8], &[u8])]) -> Result<bool, PrecompileError> {
        if pairs.is_empty() {
            return Ok(true);
        }
        let mut g1_points = Vec::with_capacity(pairs.len());
        let mut g2_points = Vec::with_capacity(pairs.len());

        for (g1_bytes, g2_bytes) in pairs {
            let g1 = read_g1_point(g1_bytes)?;
            let g2 = read_g2_point(g2_bytes)?;

            let (g1_x, g1_y) = g1.into_coords();
            let g1 = AffinePoint::new(g1_x, g1_y);

            let (g2_x, g2_y) = g2.into_coords();
            let g2 = AffinePoint::new(g2_x, g2_y);

            g1_points.push(g1);
            g2_points.push(g2);
        }

        let pairing_result = Bn254::pairing_check(&g1_points, &g2_points).is_ok();
        Ok(pairing_result)
    }

    /// Custom secp256k1 ECDSA signature recovery with openvm optimization
    fn secp256k1_ecrecover(
        &self,
        sig_bytes: &[u8; 64],
        mut recid: u8,
        msg_hash: &[u8; 32],
    ) -> Result<[u8; 32], PrecompileError> {
        let mut sig = Signature::from_slice(sig_bytes)
            .map_err(|_| PrecompileError::other("Invalid signature format"))?;

        if let Some(sig_normalized) = sig.normalize_s() {
            sig = sig_normalized;
            recid ^= 1;
        }

        let recovery_id = RecoveryId::from_byte(recid)
            .ok_or_else(|| PrecompileError::other("Invalid recovery ID"))?;

        let recovered_key =
            VerifyingKey::recover_from_prehash_noverify(msg_hash, &sig.to_bytes(), recovery_id)
                .map_err(|_| PrecompileError::other("Key recovery failed"))?;

        let public_key = recovered_key.as_affine();
        let mut encoded_pubkey = [0u8; 64];
        encoded_pubkey[..32].copy_from_slice(&WeierstrassPoint::x(public_key).to_be_bytes());
        encoded_pubkey[32..].copy_from_slice(&WeierstrassPoint::y(public_key).to_be_bytes());

        let pubkey_hash = keccak256(&encoded_pubkey);
        let mut address = [0u8; 32];
        address[12..].copy_from_slice(&pubkey_hash[12..]);

        Ok(address)
    }

    /// Custom KZG point evaluation with configurable backends
    fn verify_kzg_proof(
        &self,
        z: &[u8; 32],
        y: &[u8; 32],
        commitment: &[u8; 48],
        proof: &[u8; 48],
    ) -> Result<(), PrecompileError> {
        let env = openvm_kzg::EnvKzgSettings::default();
        let kzg_settings = env.get();

        let commitment_bytes = Bytes48::from_slice(commitment)
            .map_err(|_| PrecompileError::other("invalid commitment bytes"))?;
        let z_bytes =
            Bytes32::from_slice(z).map_err(|_| PrecompileError::other("invalid z bytes"))?;
        let y_bytes =
            Bytes32::from_slice(y).map_err(|_| PrecompileError::other("invalid y bytes"))?;
        let proof_bytes = Bytes48::from_slice(proof)
            .map_err(|_| PrecompileError::other("invalid proof bytes"))?;

        KzgProof::verify_kzg_proof(
            &commitment_bytes,
            &z_bytes,
            &y_bytes,
            &proof_bytes,
            kzg_settings,
        )
        .map_err(|_| PrecompileError::other("openvm kzg proof verification failed"))?;
        Ok(())
    }
}

/// Install OpenVM crypto implementations globally
pub fn install_openvm_crypto() -> Result<bool, Box<dyn std::error::Error>> {
    // Install OpenVM k256 provider for Alloy (transaction validation)
    install_default_provider(Arc::new(OpenVmK256Provider))?;

    // Install OpenVM crypto for REVM precompiles
    let installed = install_crypto(OpenVmCrypto);

    Ok(installed)
}

// Helper functions for BN254 operations

#[inline]
fn read_fq(input: &[u8]) -> Result<Fp, PrecompileError> {
    if input.len() < FQ_LEN {
        Err(PrecompileError::Bn254FieldPointNotAMember)
    } else {
        Fp::from_be_bytes(&input[..FQ_LEN]).ok_or(PrecompileError::Bn254FieldPointNotAMember)
    }
}

#[inline]
fn read_fq2(input: &[u8]) -> Result<Fp2, PrecompileError> {
    let y = read_fq(&input[..FQ_LEN])?;
    let x = read_fq(&input[FQ_LEN..FQ_LEN * 2])?;
    Ok(Fp2::new(x, y))
}

#[inline]
fn read_g1_point(input: &[u8]) -> Result<G1Affine, PrecompileError> {
    if input.len() != G1_LEN {
        return Err(PrecompileError::Bn254PairLength);
    }
    let px = read_fq(&input[0..FQ_LEN])?;
    let py = read_fq(&input[FQ_LEN..G1_LEN])?;
    G1Affine::from_xy(px, py).ok_or(PrecompileError::Bn254AffineGFailedToCreate)
}

#[inline]
fn read_g2_point(input: &[u8]) -> Result<G2Affine, PrecompileError> {
    if input.len() != G2_LEN {
        return Err(PrecompileError::Bn254PairLength);
    }
    let c0 = read_fq2(&input[0..G1_LEN])?;
    let c1 = read_fq2(&input[G1_LEN..G2_LEN])?;
    G2Affine::from_xy(c0, c1).ok_or(PrecompileError::Bn254AffineGFailedToCreate)
}

#[inline]
fn encode_g1_point(point: G1Affine) -> [u8; G1_LEN] {
    let mut output = [0u8; G1_LEN];

    let x_bytes: &[u8] = point.x().as_le_bytes();
    let y_bytes: &[u8] = point.y().as_le_bytes();
    for i in 0..FQ_LEN {
        output[i] = x_bytes[FQ_LEN - 1 - i];
        output[i + FQ_LEN] = y_bytes[FQ_LEN - 1 - i];
    }
    output
}

/// Reads a scalar from the input slice
///
/// Note: The scalar does not need to be canonical.
///
/// # Panics
///
/// If `input.len()` is not equal to [`SCALAR_LEN`].
#[inline]
fn read_scalar(input: &[u8]) -> Scalar {
    assert_eq!(
        input.len(),
        SCALAR_LEN,
        "unexpected scalar length. got {}, expected {SCALAR_LEN}",
        input.len()
    );
    Scalar::from_be_bytes_unchecked(input)
}
