//! Guest program input generation and metadata types

use anyhow::Context;
use ere_io::Io;
use ere_platform_trait::Platform;
use ere_zkvm_interface::zkvm::Input;
use guest_libs::guest::{Guest, GuestInput, GuestOutput};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{fmt::Debug, marker::PhantomData, ops::Deref, sync::OnceLock};

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

#[derive(Debug)]
pub struct GenericGuestFixture2<M> {
    /// The name of the guest program fixture.
    pub name: String,
    /// The input to be provided to the guest program fixture.
    pub input: Input,
    /// The expected public values of guest program.
    expected_public_values: Vec<u8>,
    /// Associated metadata for the guest program fixture.
    pub metadata: M,
}

impl<M> GenericGuestFixture2<M>
where
    M: 'static + Send + Sync + Serialize,
{
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

impl<M> GuestFixture for GenericGuestFixture2<M>
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

    /// Converts this [`GenericGuestFixture`] into a [`OutputHashedGuestFixture`]
    /// with [`Sha256`].
    pub const fn into_output_sha256(self) -> OutputHashedGuestFixture<Self, Sha256> {
        OutputHashedGuestFixture::new(self)
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
                    fn read_whole_input() -> impl Deref<Target = [u8]> {
                        panic!("`Guest::compute` should not invoke `Platform::read_whole_input`");
                        #[allow(unreachable_code)]
                        Vec::new() // For `impl Deref<Target = [u8]>` to know the concrete type.
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

    fn input(&self) -> anyhow::Result<Input> {
        let stdin = G::Io::serialize_input(&self.input()).context("Failed to serialize input")?;
        Ok(Input::new().with_prefixed_stdin(stdin))
    }

    fn expected_public_values(&self) -> anyhow::Result<Vec<u8>> {
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

    fn input(&self) -> anyhow::Result<Input> {
        self.inner.input()
    }

    fn expected_public_values(&self) -> anyhow::Result<Vec<u8>> {
        Ok(D::digest(self.inner.expected_public_values()?).to_vec())
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
