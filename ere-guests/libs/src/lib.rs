//! Library for guest programs containing shared types and utilities.
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use core::ops::Deref;

use reth_ethereum_primitives::Block;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub mod block_ssz;
pub mod mpt;

/// Block wrapper that supports bincode serialization
#[serde_as]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BincodeBlock(
    #[serde_as(
        as = "reth_primitives_traits::serde_bincode_compat::Block<reth_ethereum_primitives::TransactionSigned, alloy_consensus::Header>"
    )]
    pub Block,
);

impl Deref for BincodeBlock {
    type Target = Block;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
