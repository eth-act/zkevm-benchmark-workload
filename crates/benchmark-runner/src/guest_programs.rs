//! Guest program input generation and metadata types

use ere_io::Io;
use ere_zkvm_interface::zkvm::Input;
use serde::Serialize;
use sha2::{Digest, Sha256};
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
    /// Creates a new [`GenericGuestFixture`] from a guest input, output, and metadata.
    pub fn new<G: ere_guests_guest::Guest>(
        name: impl AsRef<str>,
        input: ere_guests_guest::GuestInput<G>,
        output: ere_guests_guest::GuestOutput<G>,
        metadata: M,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            input: Input::new().with_prefixed_stdin(G::Io::serialize_input(&input).unwrap()),
            expected_public_values: G::Io::serialize_output(&output).unwrap(),
            metadata,
        }
    }

    /// Consumes the [`GericGuestFixture`] and constructs a new one with sha256 output.
    pub fn output_sha256(mut self) -> Self {
        self.expected_public_values = Sha256::digest(self.expected_public_values).to_vec();
        self
    }

    /// Converts this [`OutputHashedGuestFixture`] into a boxed [`GuestFixture`] trait
    /// object.
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
