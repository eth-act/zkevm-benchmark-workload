//! This module provides trait for guest program abstraction, that can also be
//! shared between Rust guest and host.

use ere_io::Io;
use ere_platform_trait::Platform;

/// Guest program that can be ran given [`Platform`] implementation.
pub trait Guest: Clone {
    /// The I/O type that defines input and output serialization for this guest program.
    type Io: Io;

    /// Executes the core computation logic of the guest program.
    ///
    /// This method takes the deserialized input and produces the output for the guest program.
    /// It is called by [`Guest::run`] after reading and deserializing the input.
    fn compute<P: Platform>(input: <Self::Io as Io>::Input) -> <Self::Io as Io>::Output;

    /// Runs the complete guest program workflow: reads input, computes output, and writes output.
    ///
    /// This is the main entry point for executing a guest program. It:
    /// 1. Reads the input with the platform and deserializes it using [`Io::deserialize_input`]
    /// 2. Calls [`Guest::compute`] to process the input
    /// 3. Serializes the output using [`Io::serialize_output`] and writes it with the platform
    fn run<P: Platform>() {
        let input = P::cycle_scope("read_input", || {
            let input_bytes = P::read_whole_input();
            Self::Io::deserialize_input(&input_bytes).unwrap()
        });

        let output = Self::compute::<P>(input);

        P::cycle_scope("write_output", || {
            let output_bytes = Self::Io::serialize_output(&output).unwrap();
            P::write_whole_output(&output_bytes);
        });
    }
}

/// Associated type `Io` of [`Guest`].
pub type GuestIo<G> = <G as Guest>::Io;

/// Associated type `Input` of [`Guest::Io`].
pub type GuestInput<G> = <GuestIo<G> as Io>::Input;

/// Associated type `Output` of [`Guest::Io`].
pub type GuestOutput<G> = <GuestIo<G> as Io>::Output;

/// Associated type `Error` of [`Guest::Io`].
pub type GuestError<G> = <GuestIo<G> as Io>::Error;
