//! Amsterdam chain configuration used by generated stateless inputs.

use stateless_validator_common::guest::input::{
    BlobSchedule, ChainConfig, ForkActivation, ForkConfig, ProtocolFork,
};

pub(crate) fn amsterdam(chain_id: u64) -> ChainConfig {
    ChainConfig {
        chain_id,
        active_fork: ForkConfig::new(
            ProtocolFork::Amsterdam,
            ForkActivation::new(None, Some(0)),
            Some(BlobSchedule {
                target: 14,
                max: 21,
                base_fee_update_fraction: 11_684_671,
            }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_amsterdam_spec_defaults() {
        let cfg = amsterdam(1);

        assert_eq!(cfg.chain_id, 1);
        assert_eq!(cfg.active_fork.fork, ProtocolFork::Amsterdam);
        assert_eq!(cfg.active_fork.activation.block_number(), None);
        assert_eq!(cfg.active_fork.activation.timestamp(), Some(0));
        let schedule = cfg.active_fork.blob_schedule().unwrap();
        assert_eq!(schedule.target, 14);
        assert_eq!(schedule.max, 21);
        assert_eq!(schedule.base_fee_update_fraction, 11_684_671);
    }
}
