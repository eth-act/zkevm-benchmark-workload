//! Amsterdam chain configuration used by generated stateless inputs.

use stateless_validator_common::guest::input::{ChainConfig, ForkActivation, ForkConfig};

pub(crate) fn amsterdam(chain_id: u64) -> ChainConfig {
    ChainConfig {
        chain_id,
        active_fork: ForkConfig::new(ForkActivation::new(None, Some(0))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_amsterdam_spec_defaults() {
        let cfg = amsterdam(1);

        assert_eq!(cfg.chain_id, 1);
        assert_eq!(cfg.active_fork.activation.block_number(), None);
        assert_eq!(cfg.active_fork.activation.timestamp(), Some(0));
    }
}
