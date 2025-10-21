//! Pico guest program

#![no_main]

extern crate alloc;

use kzg_rs::{Bytes32, Bytes48};
use pico_sdk::io::{commit_bytes, read_vec};
use reth_guest::{
    guest::ethereum_guest,
    sdk::{ScopeMarker, SDK},
};
use revm::precompile::{interface::install_crypto, Crypto, PrecompileError};

pico_sdk::entrypoint!(main);

struct PicoSDK;

impl SDK for PicoSDK {
    fn read_input() -> Vec<u8> {
        read_vec()
    }

    fn commit_output(output: [u8; 32]) {
        commit_bytes(&output);
    }

    fn cycle_scope(scope: ScopeMarker, message: &str) {
        match scope {
            ScopeMarker::Start => {
                println!("cycle-tracker-report-start: {message}")
            }
            ScopeMarker::End => {
                println!("cycle-tracker-report-end: {message}")
            }
        }
    }
}

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
    ethereum_guest::<PicoSDK>();
}
