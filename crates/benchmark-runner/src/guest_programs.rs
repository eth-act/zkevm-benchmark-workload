//! Guest program input generation and metadata types

use ere_dockerized::{zkVMKind, Input};
use serde::Serialize;
use std::fmt::Debug;

/// Trait for a guest program fixture with associated metadata.
#[auto_impl::auto_impl(&, Box)]
pub trait GuestFixture: Sync + Send {
    /// Returns the name of the guest program fixture.
    fn name(&self) -> String;

    /// Returns the metadata associated with this guest program fixture as a JSON value.
    fn metadata(&self) -> serde_json::Value;

    /// Returns [`Input`] of the guest program fixture.
    fn input(&self) -> anyhow::Result<Input>;

    /// Returns the expected public values of guest program fixture.
    fn expected_public_values(&self) -> anyhow::Result<Vec<u8>>;

    /// Returns the expected public values normalized for the selected zkVM.
    fn expected_public_values_for_zkvm(&self, zkvm_kind: zkVMKind) -> anyhow::Result<Vec<u8>> {
        Ok(normalize_expected_public_values(
            zkvm_kind,
            self.expected_public_values()?,
        ))
    }

    /// Verifies that the provided `public_values` match the expected output.
    fn verify_public_values(&self, public_values: &[u8]) -> anyhow::Result<OutputVerifierResult> {
        let expected_public_values = self.expected_public_values()?;
        Ok(if expected_public_values == public_values {
            OutputVerifierResult::Match
        } else {
            OutputVerifierResult::Mismatch(format!(
                "Public values mismatch: expected {expected_public_values:?}, got {public_values:?}",
            ))
        })
    }
}

/// A generic guest fixture containing the input, expected output, and metadata.
#[derive(Debug)]
pub struct GenericGuestFixture<M> {
    /// The name of the guest program fixture.
    pub name: String,
    /// The input to be provided to the guest program fixture.
    pub input: Input,
    /// The expected public values of guest program.
    pub expected_public_values: Vec<u8>,
    /// Associated metadata for the guest program fixture.
    pub metadata: M,
}

impl<M> GenericGuestFixture<M>
where
    M: 'static + Send + Sync + Serialize,
{
    /// Consumes the [`GenericGuestFixture`] and returns it as a boxed trait object.
    pub fn into_boxed(self) -> Box<dyn GuestFixture> {
        Box::new(self)
    }
}

impl<M> GuestFixture for GenericGuestFixture<M>
where
    M: 'static + Send + Sync + Serialize,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::to_value(&self.metadata).unwrap()
    }

    fn input(&self) -> anyhow::Result<Input> {
        Ok(self.input.clone())
    }

    fn expected_public_values(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.expected_public_values.clone())
    }
}

/// Result of output verification
#[derive(Debug)]
pub enum OutputVerifierResult {
    /// Output matches the expected result
    Match,
    /// Output does not match the expected result
    Mismatch(String),
}

fn normalize_expected_public_values(
    zkvm_kind: zkVMKind,
    mut expected_public_values: Vec<u8>,
) -> Vec<u8> {
    if matches!(zkvm_kind, zkVMKind::Airbender | zkVMKind::OpenVM)
        && expected_public_values.len() < 32
    {
        expected_public_values.resize(32, 0);
    }

    if matches!(zkvm_kind, zkVMKind::Zisk) && expected_public_values.len() < 256 {
        expected_public_values.resize(256, 0);
    }

    expected_public_values
}
