//! Pico guest program

#![no_main]

use ere_platform_pico::{PicoPlatform, pico_sdk};
use kzg_rs::{Bytes32, Bytes48};
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use revm::precompile::{Crypto, PrecompileError, interface::install_crypto};
use sha2::Sha256;

pico_sdk::entrypoint!(main);

#[derive(Debug)]
struct CryptoProvider;

// See https://github.com/bluealloy/revm/blob/e42a93a86580da9c861e568f24d86482532f3560/crates/precompile/src/kzg_point_evaluation.rs#L79-L119
impl Crypto for CryptoProvider {
    fn verify_kzg_proof(
        &self,
        z: &[u8; 32],
        y: &[u8; 32],
        commitment: &[u8; 48],
        proof: &[u8; 48],
    ) -> Result<(), PrecompileError> {
        let env = kzg_rs::EnvKzgSettings::default();
        let kzg_settings = env.get();
        if !kzg_rs::KzgProof::verify_kzg_proof(
            as_bytes48(commitment),
            as_bytes32(z),
            as_bytes32(y),
            as_bytes48(proof),
            kzg_settings,
        )
        .unwrap_or(false)
        {
            return Err(PrecompileError::BlobVerifyKzgProofFailed);
        }

        Ok(())
    }
}

/// Convert a slice to an array of a specific size.
#[inline]
fn as_array<const N: usize>(bytes: &[u8]) -> &[u8; N] {
    bytes.try_into().expect("slice with incorrect length")
}

/// Convert a slice to a 32 byte big endian array.
#[inline]
fn as_bytes32(bytes: &[u8]) -> &Bytes32 {
    // SAFETY: `#[repr(C)] Bytes32([u8; 32])`
    unsafe { &*as_array::<32>(bytes).as_ptr().cast() }
}

/// Convert a slice to a 48 byte big endian array.
#[inline]
fn as_bytes48(bytes: &[u8]) -> &Bytes48 {
    // SAFETY: `#[repr(C)] Bytes48([u8; 48])`
    unsafe { &*as_array::<48>(bytes).as_ptr().cast() }
}

/// Entry point.
pub fn main() {
    install_crypto(CryptoProvider);
    RethStatelessValidatorGuest::run::<PicoPlatform<Sha256>>();
}
