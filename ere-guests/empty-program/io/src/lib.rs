//! Input module for the empty program guest.

use guest_libs::io::{ProgramInput, ProgramInputError};

/// Input representation for a program that takes no inputs.
#[derive(Debug, Clone)]
pub struct EmptyProgramInput;

impl ProgramInput for EmptyProgramInput {
    fn serialize_inputs(&self) -> Result<Vec<u8>, ProgramInputError> {
        Ok(vec![])
    }
}
