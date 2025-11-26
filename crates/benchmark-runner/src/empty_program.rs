//! Empty program guest program.

use anyhow::Result;
use ere_dockerized::zkVMKind;

use crate::guest_programs::{GuestIO, GuestMetadata, OutputVerifier, OutputVerifierResult};

// This implementation is required since the empty program does not have any metadata.
impl GuestMetadata for () {}

/// Generate inputs for the empty program guest program.
pub fn empty_program_input() -> Result<GuestIO<(), ProgramOutputVerifier>> {
    Ok(GuestIO {
        name: "empty_program".to_string(),
        input: vec![],
        output: ProgramOutputVerifier,
        metadata: (),
    })
}

/// Verifies the output of the program.
#[derive(Debug, Clone)]
pub struct ProgramOutputVerifier;

impl OutputVerifier for ProgramOutputVerifier {
    fn check_serialized(&self, zkvm: zkVMKind, bytes: &[u8]) -> Result<OutputVerifierResult> {
        match zkvm {
            zkVMKind::SP1 | zkVMKind::Risc0 | zkVMKind::Zisk | zkVMKind::Pico => {
                match bytes.is_empty() {
                    true => Ok(OutputVerifierResult::Match),
                    false => Ok(OutputVerifierResult::Mismatch(format!(
                        "Expected empty output, got {bytes:?}",
                    ))),
                }
            }

            zkVMKind::OpenVM => match bytes == [0x00; 32] {
                true => Ok(OutputVerifierResult::Match),
                false => Ok(OutputVerifierResult::Mismatch(format!(
                    "Expected [0x00; 32], got {bytes:?}"
                ))),
            },
            _ => todo!("Output verification not implemented for this zkVM"),
        }
    }
}
