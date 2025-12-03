//! Guest program input generation and metadata types

use anyhow::Context;
use ere_io::Io;
use ere_platform_trait::Platform;
use guest_libs::guest::{Guest, GuestInput, GuestOutput};
use serde::Serialize;
use sha2::Digest;
use std::{fmt::Debug, marker::PhantomData, sync::OnceLock};

/// Trait for a guest program fixture with associated metadata.
#[auto_impl::auto_impl(&, Box)]
pub trait GuestFixture: Sync + Send {
    /// Returns the name of the guest program fixture.
    fn name(&self) -> String;

    /// Returns the metadata associated with this guest program fixture as a JSON value.
    fn metadata(&self) -> serde_json::Value;

    /// Returns serialized input of the guest program fixture.
    fn serialized_input(&self) -> anyhow::Result<Vec<u8>>;

    /// Returns serialized output of the guest program fixture.
    fn serialized_output(&self) -> anyhow::Result<Vec<u8>>;

    /// Verifies that the provided `public_values` match the expected output.
    fn verify_public_values(&self, public_values: &[u8]) -> anyhow::Result<OutputVerifierResult> {
        let serialized_output = self.serialized_output()?;
        Ok(if serialized_output == public_values {
            OutputVerifierResult::Match
        } else {
            OutputVerifierResult::Mismatch(format!(
                "Public values mismatch: expected {serialized_output:?}, got {public_values:?}"
            ))
        })
    }
}

/// A generic implementation of `GuestFixture` that wraps a guest program fixture
/// with its input, output, and metadata.
#[derive(Debug)]
pub struct GenericGuestFixture<G: Guest, M> {
    /// The name of the guest program fixture.
    pub name: String,
    /// The input to be provided to the guest program fixture.
    pub input: GuestInput<G>,
    /// The expected output for the guest program fixture.
    ///
    /// If `None` is given, `G::compute` will be used to compute the output.
    pub output: OnceLock<GuestOutput<G>>,
    /// Associated metadata for the guest program fixture.
    pub metadata: M,
}

impl<G, M> GenericGuestFixture<G, M>
where
    G: 'static + Guest,
    M: 'static + Send + Sync + Serialize,
{
    /// Converts this [`GenericGuestFixture`] into a boxed [`GuestFixture`] trait object.
    pub fn into_boxed(self) -> Box<dyn GuestFixture> {
        Box::new(self)
    }

    /// Returns the guest program input.
    fn input(&self) -> GuestInput<G> {
        self.input.clone()
    }

    /// Returns the guest program output, computing it if not provided.
    fn output(&self) -> GuestOutput<G> {
        self.output
            .get_or_init(|| {
                struct HostPlatform;

                impl Platform for HostPlatform {
                    fn read_whole_input() -> Vec<u8> {
                        panic!("`Guest::compute` should not invoke `Platform::read_whole_input`")
                    }

                    fn write_whole_output(_: &[u8]) {
                        panic!("`Guest::compute` should not invoke `Platform::write_whole_output`")
                    }

                    fn print(message: &str) {
                        print!("{message}");
                    }
                }

                G::compute::<HostPlatform>(self.input())
            })
            .clone()
    }
}

impl<G, M> GuestFixture for GenericGuestFixture<G, M>
where
    G: 'static + Guest,
    M: 'static + Send + Sync + Serialize,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::to_value(&self.metadata).unwrap()
    }

    fn serialized_input(&self) -> anyhow::Result<Vec<u8>> {
        G::Io::serialize_input(&self.input()).context("Failed to serialize input")
    }

    fn serialized_output(&self) -> anyhow::Result<Vec<u8>> {
        G::Io::serialize_output(&self.output()).context("Failed to serialize output")
    }
}

/// A wrapper around a `GuestFixture` expects the output is hashed.
///
/// This is useful when the guest program outputs a hash of the result instead of the full result.
#[derive(Debug)]
pub struct OutputHashedGuestFixture<G: GuestFixture, D> {
    inner: G,
    _marker: PhantomData<D>,
}

impl<G, D> OutputHashedGuestFixture<G, D>
where
    G: 'static + GuestFixture,
    D: 'static + Send + Sync + Digest,
{
    /// Creates a new [`OutputHashedGuestFixture`] wrapping the given [`GuestFixture`].
    pub const fn new(inner: G) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Converts this [`OutputHashedGuestFixture`] into a boxed [`GuestFixture`] trait
    /// object.
    pub fn into_boxed(self) -> Box<dyn GuestFixture> {
        Box::new(self)
    }
}

impl<G, D> GuestFixture for OutputHashedGuestFixture<G, D>
where
    G: 'static + GuestFixture,
    D: 'static + Send + Sync + Digest,
{
    fn name(&self) -> String {
        self.inner.name()
    }

    fn metadata(&self) -> serde_json::Value {
        self.inner.metadata()
    }

    fn serialized_input(&self) -> anyhow::Result<Vec<u8>> {
        self.inner.serialized_input()
    }

    fn serialized_output(&self) -> anyhow::Result<Vec<u8>> {
        self.inner.serialized_output()
    }

    fn verify_public_values(&self, public_values: &[u8]) -> anyhow::Result<OutputVerifierResult> {
        let hashed_output = D::digest(self.serialized_output()?);
        Ok(if hashed_output.as_slice() == public_values {
            OutputVerifierResult::Match
        } else {
            OutputVerifierResult::Mismatch(format!(
                "Public values hash mismatch: expected {hashed_output:?}, got {public_values:?}"
            ))
        })
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
