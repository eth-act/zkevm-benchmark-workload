//! Stateless validator guest program.

mod eest;
mod fixtures;
mod inputs;

use crate::{guest_programs::GuestFixture, runner::Action};
use anyhow::{bail, Context, Result};
use ere_dockerized::zkVMKind;
use stateless_validator_catalog::StatelessValidatorKind;
use std::path::Path;
use strum::{AsRefStr, EnumString};

pub use fixtures::{benchmark_fixture_paths, iter_benchmark_fixture_paths};

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
    /// Zesu stateless block validation guest program.
    Zesu,
    /// Nimbus stateless block validation guest program.
    Nimbus,
}

impl ExecutionClient {
    /// Returns the active upstream guest kind for this execution client.
    pub fn registered_kind(self) -> Result<StatelessValidatorKind> {
        let kind = match self {
            Self::Reth => StatelessValidatorKind::Reth,
            Self::Ethrex => StatelessValidatorKind::Ethrex,
            Self::Zesu => StatelessValidatorKind::Zesu,
            Self::Nimbus => StatelessValidatorKind::Nimbus,
        };
        if kind.version().is_none() {
            bail!(
                "{0} is temporarily unavailable: the pinned ere-guests catalog has no {0} guest release",
                self.as_ref()
            );
        }
        Ok(kind)
    }

    /// Rejects combinations without release artifacts before loading a guest.
    pub fn validate_zkvm(self, zkvm: zkVMKind) -> Result<()> {
        self.registered_kind()?;
        if matches!(self, Self::Zesu | Self::Nimbus) && zkvm != zkVMKind::Zisk {
            bail!("{} supports only ZisK; requested {zkvm}", self.as_ref());
        }
        Ok(())
    }

    /// Returns the version string associated with the selected guest artifact.
    pub fn version(self) -> Result<&'static str> {
        self.registered_kind()?
            .version()
            .context("active upstream guest is missing a version")
    }
}

/// Lazily prepares stateless validator inputs from a fixture folder.
pub fn stateless_validator_input_iter(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
    el: ExecutionClient,
    existing_output: Option<(&Path, Action)>,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>>> {
    fixtures::stateless_validator_input_iter(input_folder, selected_fixtures, el, existing_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_guests_follow_upstream_catalog() {
        assert_eq!(
            ExecutionClient::Reth.registered_kind().unwrap(),
            StatelessValidatorKind::Reth
        );
        assert_eq!(
            ExecutionClient::Ethrex.registered_kind().unwrap(),
            StatelessValidatorKind::Ethrex
        );
        assert_eq!(ExecutionClient::Reth.version().unwrap(), "0.1.0-rc.4");
        assert_eq!(ExecutionClient::Ethrex.version().unwrap(), "29.0.0");
        assert_eq!(ExecutionClient::Nimbus.version().unwrap(), "v0.2.1-alpha");
        for zkvm in [zkVMKind::OpenVM, zkVMKind::SP1, zkVMKind::Zisk] {
            assert!(ExecutionClient::Reth.validate_zkvm(zkvm).is_ok());
            assert!(ExecutionClient::Ethrex.validate_zkvm(zkvm).is_ok());
            assert_eq!(
                ExecutionClient::Nimbus.validate_zkvm(zkvm).is_ok(),
                zkvm == zkVMKind::Zisk
            );
        }
    }

    // TODO(tests-zkevm@v21): move Zesu back to the active guests once it publishes a v21 release
    // and ere-guests registers it.
    #[test]
    fn unreleased_guests_are_temporarily_unavailable() {
        let client = ExecutionClient::Zesu;
        let expected = format!(
            "{0} is temporarily unavailable: the pinned ere-guests catalog has no {0} guest release",
            client.as_ref()
        );
        assert_eq!(client.version().unwrap_err().to_string(), expected);
        for zkvm in [zkVMKind::OpenVM, zkVMKind::SP1, zkVMKind::Zisk] {
            assert_eq!(
                client.validate_zkvm(zkvm).unwrap_err().to_string(),
                expected
            );
        }
    }
}
