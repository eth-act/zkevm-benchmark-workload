//! Empty program guest program.

use crate::guest_programs::{GenericGuestFixture, GuestFixture};
use ere_zkvm_interface::Input;

/// Generate inputs for the empty program guest program.
pub fn empty_program_input() -> anyhow::Result<Box<dyn GuestFixture>> {
    let fixture = GenericGuestFixture {
        name: "empty_program".to_owned(),
        input: Input::new(),
        expected_public_values: Vec::new(),
        metadata: (),
    };
    Ok(Box::new(fixture))
}
