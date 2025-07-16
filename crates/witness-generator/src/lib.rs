#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// API definitions for generating blocks and witnesses.
mod blocks_and_witnesses;
/// Generate block and witnesses from test fixtures
pub mod eest_generator;
/// Generate block and witnesses from an RPC endpoint
pub mod rpc_generator;

pub use blocks_and_witnesses::{BlockAndWitness, BwError, WitnessGenerator};

// TEMP(PR): prob reth_stateless could re-export this inside blocks_and_witnessses module
pub use reth_ethereum_primitives::Block;
