//! CLI entry point for generating canonical stateless input bytes from network RPC data.

#![allow(
    unused_crate_dependencies,
    reason = "library dependencies are compiled separately from this CLI target"
)]

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use alloy_primitives::hex;
use clap::{Parser, ValueEnum};
use witness_generator_spec_cli::{BlockSelector, NetworkWitnessClient, NetworkWitnessConfig};

#[derive(Debug, Parser)]
#[command(
    name = "witness-generator-spec-cli",
    about = "Generate canonical Amsterdam stateless guest input bytes from CL/EL RPC endpoints.",
    long_about = None
)]
struct Cli {
    /// Consensus-layer Beacon API endpoint.
    #[arg(long)]
    cl_url: String,
    /// Execution-layer JSON-RPC endpoint.
    #[arg(long)]
    el_url: String,
    /// Beacon API block id: head, finalized, slot, or block root. Defaults to head.
    #[arg(long)]
    block_id: Option<String>,
    /// Execution block number to resolve to a CL slot via block timestamp.
    #[arg(long, conflicts_with = "block_id")]
    execution_block_number: Option<u64>,
    /// Output encoding.
    #[arg(long, value_enum, default_value_t = OutputFormat::Hex)]
    format: OutputFormat,
    /// Output file. Stdout is used when omitted.
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum OutputFormat {
    Hex,
    Raw,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let selector = match cli.execution_block_number {
        Some(number) => BlockSelector::ExecutionBlockNumber(number),
        None => match cli.block_id.as_deref().unwrap_or("head") {
            "head" => BlockSelector::Head,
            other => BlockSelector::BeaconBlockId(other.to_owned()),
        },
    };

    let client = NetworkWitnessClient::new(NetworkWitnessConfig::new(cli.cl_url, cli.el_url))?;
    let generated = client.stateless_input_bytes(selector).await?;
    let output = match cli.format {
        OutputFormat::Hex => format!("0x{}\n", hex::encode(&generated.bytes)).into_bytes(),
        OutputFormat::Raw => generated.bytes,
    };

    if let Some(path) = cli.out {
        fs::write(path, output)?;
    } else {
        io::stdout().write_all(&output)?;
    }

    Ok(())
}
