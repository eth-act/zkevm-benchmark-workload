//! Provides types and functions for stateless execution of Ethereum blocks without validation.
//!
//! This crate enables pure EVM execution for benchmarking purposes, skipping all
//! pre-execution validation and post-execution consensus checks. This is useful for
//! measuring the raw EVM execution cycles in zkVMs without the overhead of validation.
//!
//! # Key Differences from `reth-stateless`
//!
//! Unlike `reth_stateless::stateless_validation_with_trie`, this crate:
//! - **Skips** pre-execution consensus validation (header checks, ancestor verification)
//! - **Skips** post-execution consensus checks (receipts root, gas used validation)
//! - **Skips** state root verification (both pre-state and post-state)
//! - **Only** performs EVM transaction execution
//!
//! # Usage
//!
//! The primary entry point is `stateless_execution_with_trie`. It returns a boolean
//! indicating whether EVM execution succeeded.
//!
//! For zkVM guest programs, use the [`guest::RethStatelessExecutorGuest`] implementation.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![no_std]

extern crate alloc;

mod execution;
/// Guest program implementation for zkVMs.
pub mod guest;
mod witness_db;

pub use execution::{stateless_execution_with_trie, StatelessExecutionError};

// Re-export types that users will need
pub use reth_stateless::{ExecutionWitness, Genesis, StatelessInput, UncompressedPublicKey};

