//! SP1 guest program

#![no_main]

extern crate alloc;
use alloc::sync::Arc;

use guest_libs::mpt::SparseState;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{Genesis, StatelessInput, stateless_validation_with_trie};
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    init_tracing_just_like_println();

    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read::<StatelessInput>();
    let genesis = Genesis {
        config: input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: public_inputs_preparation");
    let header = input.block.header().clone();
    let parent_hash = input.block.parent_hash;
    println!("cycle-tracker-report-end: public_inputs_preparation");

    println!("cycle-tracker-report-start: validation");
    let res = stateless_validation_with_trie::<SparseState, _, _>(
        input.block,
        input.witness,
        chain_spec,
        evm_config,
    );
    println!("cycle-tracker-report-end: validation");

    println!("cycle-tracker-report-start: commit_public_inputs");
    // The public inputs are:
    // - block_hash : [u8;32]
    // - parent_hash : [u8;32]
    // - successful_block_validation : bool
    match res {
        Ok(block_hash) => {
            sp1_zkvm::io::commit(&block_hash.0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&true);
        }
        Err(_) => {
            sp1_zkvm::io::commit(&header.hash_slow().0);
            sp1_zkvm::io::commit(&parent_hash.0);
            sp1_zkvm::io::commit(&false);
        }
    }
    println!("cycle-tracker-report-end: commit_public_inputs");
}

/// TODO: can we put this in the host? (Note that if we want sp1 logs, it will look very plain in that case)
/// Initializes a basic `tracing` subscriber that mimics `println!` behavior.
///
/// This is because we want to use tracing in the `no_std` program to capture cycle counts.
fn init_tracing_just_like_println() {
    // Build a formatter that prints *only* the message text + '\n'
    let plain = fmt::format()
        .without_time() // no timestamp
        .with_level(false) // no INFO/TRACE prefix
        .with_target(false); // no module path

    fmt::Subscriber::builder()
        .event_format(plain) // use the stripped-down format
        .with_writer(std::io::stdout) // stdout == println!
        .with_max_level(tracing::Level::INFO) // capture info! and up
        .init(); // set as global default
}
