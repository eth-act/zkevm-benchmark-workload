//! Empty program guest program.

use anyhow::Result;
use ere_dockerized::ErezkVM;
use zkvm_interface::Input;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier};

// This implementation is required since the empty program does not have any metadata.
impl GuestMetadata for () {}

/// Generate inputs for the empty program guest program.
pub fn empty_program_input() -> GuestIO<(), ProgramOutputVerifier> {
    GuestIO {
        name: "empty_program".to_string(),
        input: Input::new(),
        output: ProgramOutputVerifier,
        metadata: (),
    }
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier;

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, _zkvm: ErezkVM, bytes: &[u8]) -> Result<bool> {
        Ok(bytes.is_empty())
    }
}
