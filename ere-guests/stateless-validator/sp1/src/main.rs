//! SP1 guest program

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![no_main]

extern crate alloc;

use alloc::sync::Arc;

use alloy_genesis::Genesis;
use reth_chainspec::{Chain, ChainSpec, ChainSpecBuilder};
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{fork_spec::ForkSpec, validation::stateless_validation, StatelessInput};
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);
/// Entry point.
pub fn main() {
    init_tracing_just_like_println();

    println!("cycle-tracker-report-start: read_input");
    let input = sp1_zkvm::io::read::<StatelessInput>();
    let fork_spec = sp1_zkvm::io::read::<ForkSpec>();
    let chain_spec: Arc<ChainSpec> = Arc::new(from(fork_spec));
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    println!("cycle-tracker-report-end: read_input");

    println!("cycle-tracker-report-start: validation");
    stateless_validation(input.block, input.witness, chain_spec, evm_config).unwrap();
    println!("cycle-tracker-report-end: validation");
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

fn from(fork_spec: ForkSpec) -> ChainSpec {
    let spec_builder = ChainSpecBuilder::default()
        .chain(Chain::mainnet())
        .genesis(Genesis::default());

    match fork_spec {
        ForkSpec::Frontier => spec_builder.frontier_activated(),
        ForkSpec::Homestead | ForkSpec::FrontierToHomesteadAt5 => {
            spec_builder.homestead_activated()
        }
        ForkSpec::EIP150 | ForkSpec::HomesteadToDaoAt5 | ForkSpec::HomesteadToEIP150At5 => {
            spec_builder.tangerine_whistle_activated()
        }
        ForkSpec::EIP158 => spec_builder.spurious_dragon_activated(),
        ForkSpec::Byzantium
        | ForkSpec::EIP158ToByzantiumAt5
        | ForkSpec::ConstantinopleFix
        | ForkSpec::ByzantiumToConstantinopleFixAt5 => spec_builder.byzantium_activated(),
        ForkSpec::Istanbul => spec_builder.istanbul_activated(),
        ForkSpec::Berlin => spec_builder.berlin_activated(),
        ForkSpec::London | ForkSpec::BerlinToLondonAt5 => spec_builder.london_activated(),
        ForkSpec::Merge
        | ForkSpec::MergeEOF
        | ForkSpec::MergeMeterInitCode
        | ForkSpec::MergePush0 => spec_builder.paris_activated(),
        ForkSpec::Shanghai => spec_builder.shanghai_activated(),
        ForkSpec::Cancun => spec_builder.cancun_activated(),
        ForkSpec::ByzantiumToConstantinopleAt5 | ForkSpec::Constantinople => {
            panic!("Overridden with PETERSBURG")
        }
        ForkSpec::Prague => spec_builder.prague_activated(),
    }
    .build()
}
