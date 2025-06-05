#![doc = include_str!("../README.md")]

mod blocks_and_witnesses;
/// generate block and witnesses from test fixtures
pub mod generate_stateless_witness;
/// generate block and witnesses from an RPC endpoint
pub mod rpc;
/// api definitions
pub mod witness_generator;

pub use blocks_and_witnesses::{BlocksAndWitnesses, BwError};
pub use reth_stateless::ClientInput;
