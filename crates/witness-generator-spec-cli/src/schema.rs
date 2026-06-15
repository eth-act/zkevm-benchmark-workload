//! Minimal canonical Amsterdam stateless input SSZ schema.

use libssz_derive::{SszDecode, SszEncode};
use libssz_types::SszList;
use stateless_validator_common::new_payload_request::NewPayloadRequestAmsterdam;

/// Canonical stateless input schema id.
pub(crate) const STATELESS_INPUT_SCHEMA_ID: u16 = 0x0001;
/// Big-endian schema id prefix bytes.
pub(crate) const STATELESS_INPUT_SCHEMA_ID_BYTES: [u8; 2] = STATELESS_INPUT_SCHEMA_ID.to_be_bytes();

pub(crate) const MAX_WITNESS_NODES: usize = 1 << 22;
pub(crate) const MAX_WITNESS_CODES: usize = 1 << 18;
pub(crate) const MAX_WITNESS_HEADERS: usize = 256;
pub(crate) const MAX_BYTES_PER_WITNESS_NODE: usize = 1 << 10;
pub(crate) const MAX_BYTES_PER_CODE: usize = 1 << 16;
pub(crate) const MAX_BYTES_PER_HEADER: usize = 1 << 10;
pub(crate) const MAX_OPTIONAL_FORK_ACTIVATION_VALUES: usize = 1;
pub(crate) const MAX_BLOB_SCHEDULES_PER_FORK: usize = 1;
pub(crate) const MAX_PUBLIC_KEYS: usize = 1 << 15;
pub(crate) const PUBLIC_KEY_BYTES: usize = 65;

pub(crate) const AMSTERDAM_PROTOCOL_FORK_INDEX: u64 = 24;
pub(crate) const AMSTERDAM_BLOB_SCHEDULE_TARGET: u64 = 14;
pub(crate) const AMSTERDAM_BLOB_SCHEDULE_MAX: u64 = 21;
pub(crate) const AMSTERDAM_BLOB_BASE_FEE_UPDATE_FRACTION: u64 = 11_684_671;

pub(crate) type WitnessNodes = SszList<SszList<u8, MAX_BYTES_PER_WITNESS_NODE>, MAX_WITNESS_NODES>;
pub(crate) type WitnessCodes = SszList<SszList<u8, MAX_BYTES_PER_CODE>, MAX_WITNESS_CODES>;
pub(crate) type WitnessHeaders = SszList<SszList<u8, MAX_BYTES_PER_HEADER>, MAX_WITNESS_HEADERS>;
pub(crate) type OptionalForkActivationValue = SszList<u64, MAX_OPTIONAL_FORK_ACTIVATION_VALUES>;
pub(crate) type OptionalBlobSchedule = SszList<SszBlobSchedule, MAX_BLOB_SCHEDULES_PER_FORK>;
pub(crate) type PublicKeys = SszList<[u8; PUBLIC_KEY_BYTES], MAX_PUBLIC_KEYS>;

/// Canonical stateless execution witness.
#[derive(Debug, Clone, PartialEq, Eq, SszEncode, SszDecode)]
pub(crate) struct SszExecutionWitness {
    /// RLP-encoded trie node preimages.
    pub(crate) state: WitnessNodes,
    /// Contract bytecode preimages.
    pub(crate) codes: WitnessCodes,
    /// RLP-encoded ancestor headers.
    pub(crate) headers: WitnessHeaders,
}

/// Canonical optional fork activation values.
#[derive(Debug, Clone, PartialEq, Eq, SszEncode, SszDecode)]
pub(crate) struct SszForkActivation {
    /// Optional activation block number.
    pub(crate) block_number: OptionalForkActivationValue,
    /// Optional activation timestamp.
    pub(crate) timestamp: OptionalForkActivationValue,
}

/// Canonical blob schedule.
#[derive(Debug, Clone, PartialEq, Eq, SszEncode, SszDecode)]
pub(crate) struct SszBlobSchedule {
    /// Target blob count.
    pub(crate) target: u64,
    /// Maximum blob count.
    pub(crate) max: u64,
    /// Blob base fee update fraction.
    pub(crate) base_fee_update_fraction: u64,
}

/// Canonical active fork config.
#[derive(Debug, Clone, PartialEq, Eq, SszEncode, SszDecode)]
pub(crate) struct SszForkConfig {
    /// Numeric protocol fork index.
    pub(crate) fork: u64,
    /// Fork activation details.
    pub(crate) activation: SszForkActivation,
    /// Optional blob schedule.
    pub(crate) blob_schedule: OptionalBlobSchedule,
}

/// Canonical compact chain config.
#[derive(Debug, Clone, PartialEq, Eq, SszEncode, SszDecode)]
pub(crate) struct SszChainConfig {
    /// Chain id.
    pub(crate) chain_id: u64,
    /// Active fork config.
    pub(crate) active_fork: SszForkConfig,
}

/// Canonical Amsterdam stateless input.
#[derive(Debug, Clone, SszEncode, SszDecode)]
pub(crate) struct SszStatelessInput {
    /// Amsterdam new-payload request.
    pub(crate) new_payload_request: NewPayloadRequestAmsterdam,
    /// Execution witness.
    pub(crate) witness: SszExecutionWitness,
    /// Compact chain config.
    pub(crate) chain_config: SszChainConfig,
    /// Uncompressed transaction public keys in payload order.
    pub(crate) public_keys: PublicKeys,
}

pub(crate) fn amsterdam_chain_config(chain_id: u64) -> anyhow::Result<SszChainConfig> {
    let activation = SszForkActivation {
        block_number: OptionalForkActivationValue::try_from(Vec::<u64>::new())
            .map_err(|err| anyhow::anyhow!("block_number optional list invalid: {err:?}"))?,
        timestamp: OptionalForkActivationValue::try_from(vec![0])
            .map_err(|err| anyhow::anyhow!("timestamp optional list invalid: {err:?}"))?,
    };

    let blob_schedule = OptionalBlobSchedule::try_from(vec![SszBlobSchedule {
        target: AMSTERDAM_BLOB_SCHEDULE_TARGET,
        max: AMSTERDAM_BLOB_SCHEDULE_MAX,
        base_fee_update_fraction: AMSTERDAM_BLOB_BASE_FEE_UPDATE_FRACTION,
    }])
    .map_err(|err| anyhow::anyhow!("blob_schedule optional list invalid: {err:?}"))?;

    Ok(SszChainConfig {
        chain_id,
        active_fork: SszForkConfig {
            fork: AMSTERDAM_PROTOCOL_FORK_INDEX,
            activation,
            blob_schedule,
        },
    })
}

#[cfg(test)]
mod tests {
    use libssz::SszEncode;

    use super::*;

    #[test]
    fn amsterdam_chain_config_uses_spec_defaults() {
        let cfg = amsterdam_chain_config(1).unwrap();

        assert_eq!(cfg.chain_id, 1);
        assert_eq!(cfg.active_fork.fork, 24);
        assert_eq!(cfg.active_fork.fork, AMSTERDAM_PROTOCOL_FORK_INDEX);
        assert!(cfg.active_fork.activation.block_number.is_empty());
        assert_eq!(
            cfg.active_fork.activation.timestamp.first().copied(),
            Some(0)
        );
        let schedule = cfg.active_fork.blob_schedule.first().unwrap();
        assert_eq!(schedule.target, 14);
        assert_eq!(schedule.max, 21);
        assert_eq!(schedule.base_fee_update_fraction, 11_684_671);
    }

    #[test]
    fn schema_prefix_is_big_endian() {
        assert_eq!(STATELESS_INPUT_SCHEMA_ID_BYTES, [0x00, 0x01]);
        assert_eq!(STATELESS_INPUT_SCHEMA_ID.to_be_bytes(), [0x00, 0x01]);
    }

    #[test]
    fn optional_values_have_distinct_encodings() {
        let absent = OptionalForkActivationValue::try_from(Vec::<u64>::new()).unwrap();
        let present = OptionalForkActivationValue::try_from(vec![0_u64]).unwrap();

        assert_ne!(absent.to_ssz(), present.to_ssz());
        assert!(absent.is_empty());
        assert_eq!(present.len(), 1);
    }
}
