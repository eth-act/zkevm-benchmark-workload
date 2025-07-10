#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// API definitions for generating blocks and witnesses.
mod blocks_and_witnesses;
/// Generate block and witnesses from test fixtures
pub mod eest_generator;
/// Generate block and witnesses from an RPC endpoint
pub mod rpc_generator;

pub use blocks_and_witnesses::{BenchmarkFixture, BwError, WitnessGenerator};
pub use reth_stateless::StatelessInput;
