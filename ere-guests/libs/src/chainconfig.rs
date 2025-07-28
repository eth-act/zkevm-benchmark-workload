//! Provides stateless-validator guest program chain configuration.

use alloy_chains::NamedChain;
use alloy_eips::{BlobScheduleBlobParams, eip6110::MAINNET_DEPOSIT_CONTRACT_ADDRESS};
use alloy_hardforks::EthereumHardfork;
use reth_stateless::{chain_spec::ChainSpec, fork_spec::ForkSpec};
use serde::{Deserialize, Serialize};

/// Describes a target chain configuration for the stateless validator.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ChainConfig {
    /// Mainnet configuration.
    Mainnet,
    /// EEST test where the fork specified and all previous ones are activated at genesis.
    Test(ForkSpec),
}

impl From<ChainConfig> for ChainSpec {
    fn from(value: ChainConfig) -> Self {
        match value {
            ChainConfig::Mainnet => Self {
                chain: NamedChain::Mainnet.into(),
                hardforks: EthereumHardfork::mainnet().into(),
                deposit_contract_address: Some(MAINNET_DEPOSIT_CONTRACT_ADDRESS),
                blob_params: BlobScheduleBlobParams::mainnet(),
            },
            ChainConfig::Test(fork) => fork.into(),
        }
    }
}
