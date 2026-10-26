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
    /// Nethermind stateless block validation guest program.
    Nethermind,
    /// Zilkworm stateless block validation guest program.
    Zilkworm,
}

impl ExecutionClient {
    /// Returns the active upstream guest kind for this execution client.
    pub fn registered_kind(self) -> Result<StatelessValidatorKind> {
        let kind = match self {
            Self::Reth => StatelessValidatorKind::Reth,
            Self::Ethrex => StatelessValidatorKind::Ethrex,
            Self::Zesu => StatelessValidatorKind::Zesu,
            Self::Nimbus => StatelessValidatorKind::Nimbus,
            Self::Nethermind => StatelessValidatorKind::Nethermind,
            Self::Zilkworm => StatelessValidatorKind::Zilkworm,
        };
        if kind.version().is_none() {
            bail!(
                "{0} is temporarily unavailable: the pinned ere-guests catalog has no {0} guest release",
                self.as_ref()
            );
        }
        Ok(kind)
    }

    /// Returns the zkVMs with release artifacts in the pinned ere-guests registry.
    const fn supported_zkvms(self) -> &'static [zkVMKind] {
        match self {
            Self::Reth | Self::Ethrex => &[zkVMKind::OpenVM, zkVMKind::SP1, zkVMKind::Zisk],
            Self::Nimbus | Self::Nethermind => &[zkVMKind::SP1, zkVMKind::Zisk],
            Self::Zilkworm => &[zkVMKind::SP1],
            // TODO(ere-guests@v0.19): Zesu has no ZisK v1.3 guest yet, so `registered_kind` rejects
            // it before this list is used.
            Self::Zesu => &[zkVMKind::Zisk],
        }
    }

    /// Rejects combinations without release artifacts before loading a guest.
    pub fn validate_zkvm(self, zkvm: zkVMKind) -> Result<()> {
        self.registered_kind()?;
        let supported = self.supported_zkvms();
        if !supported.contains(&zkvm) {
            let supported: Vec<_> = supported.iter().map(zkVMKind::as_str).collect();
            bail!(
                "{} supports only {}; requested {zkvm}",
                self.as_ref(),
                supported.join(", ")
            );
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
        let all = [zkVMKind::OpenVM, zkVMKind::SP1, zkVMKind::Zisk];
        let sp1_and_zisk = [zkVMKind::SP1, zkVMKind::Zisk];
        for (client, kind, version, zkvms) in [
            (
                ExecutionClient::Reth,
                StatelessValidatorKind::Reth,
                "0.1.0-rc.6",
                &all[..],
            ),
            (
                ExecutionClient::Ethrex,
                StatelessValidatorKind::Ethrex,
                "30.0.1",
                &all[..],
            ),
            (
                ExecutionClient::Nimbus,
                StatelessValidatorKind::Nimbus,
                "v0.3.1-alpha",
                &sp1_and_zisk[..],
            ),
            (
                ExecutionClient::Nethermind,
                StatelessValidatorKind::Nethermind,
                "2.3.0-preview+3",
                &sp1_and_zisk[..],
            ),
            (
                ExecutionClient::Zilkworm,
                StatelessValidatorKind::Zilkworm,
                "0.1.0-alpha.7",
                &[zkVMKind::SP1][..],
            ),
        ] {
            assert_eq!(client.registered_kind().unwrap(), kind);
            assert_eq!(client.version().unwrap(), version);
            for zkvm in zkVMKind::iter() {
                assert_eq!(
                    client.validate_zkvm(zkvm).is_ok(),
                    zkvms.contains(&zkvm),
                    "{client:?} {zkvm}"
                );
            }
        }
    }

    // TODO(ere-guests@v0.19): move Zesu back to the active guests once it publishes a ZisK v1.3
    // guest and ere-guests registers it.
    #[test]
    fn unreleased_guests_are_temporarily_unavailable() {
        let client = ExecutionClient::Zesu;
        let expected = format!(
            "{0} is temporarily unavailable: the pinned ere-guests catalog has no {0} guest release",
            client.as_ref()
        );
        assert_eq!(client.version().unwrap_err().to_string(), expected);
        for zkvm in zkVMKind::iter() {
            assert_eq!(
                client.validate_zkvm(zkvm).unwrap_err().to_string(),
                expected
            );
        }
    }
}
