//! Empty program guest program.

use crate::guest_programs::{GuestFixture, OutputVerifierResult};
use ere_zkvm_interface::Input;

/// Empty program guest program.
#[derive(Debug, Clone)]
pub struct EmptyGuestFixture;

impl GuestFixture for EmptyGuestFixture {
    fn name(&self) -> String {
        "empty_program".to_string()
    }

    fn metadata(&self) -> serde_json::Value {
        serde_json::Value::default()
    }

    fn input(&self) -> anyhow::Result<Input> {
        Ok(Input::new())
    }

    fn expected_public_values(&self) -> anyhow::Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn verify_public_values(&self, public_values: &[u8]) -> anyhow::Result<OutputVerifierResult> {
        // For OpenVM it has fixed size public values (32 bytes by default,
        // now allowed to configure to 0 bytes), so here we treat all zero as
        // valid empty output as well.
        let all_zero = public_values.iter().all(|v| *v == 0);
        Ok(match public_values.is_empty() || all_zero {
            true => OutputVerifierResult::Match,
            false => OutputVerifierResult::Mismatch(format!(
                "Expected empty output, got {public_values:?}",
            )),
        })
    }
}

/// Generate inputs for the empty program guest program.
pub fn empty_program_input() -> anyhow::Result<Box<dyn GuestFixture>> {
    Ok(Box::new(EmptyGuestFixture))
}
