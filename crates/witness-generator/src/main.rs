use clap::{Parser, Subcommand, ValueEnum};
use std::{path::PathBuf, process::Command};
use witness_generator::{
    WitnessGenerator, eest_generator::ExecSpecTestBlocksAndWitnesses,
    rpc_generator::RPCBlocksAndWitnessesBuilder,
};

#[derive(Parser)]
#[command(name = "zkvm-fixture-generator")]
#[command(about = "Generate fixtures for zkEVM benchmarking tool")]
#[command(version)]
struct Cli {
    /// Output folder for generated fixtures
    #[arg(short, long, default_value = "zkevm-fixtures-input")]
    output_folder: PathBuf,

    /// Source of blocks and witnesses
    #[command(subcommand)]
    source: SourceCommand,
}

#[derive(Subcommand, Clone, Debug)]
enum SourceCommand {
    Tests {
        // EEST release tag to use (e.g., "v0.1.0"). If empty, the latest release will be used.
        #[arg(short, long)]
        tag: Option<String>,
    },
    Rpc {
        /// Number of last blocks to pull
        #[arg(long, conflicts_with = "block")]
        last_n_blocks: Option<usize>,

        /// Specific block number to pull
        #[arg(long, conflicts_with = "last_n_blocks")]
        block: Option<u64>,

        /// RPC URL to use (mandatory)
        #[arg(long)]
        rpc_url: String,

        /// Optional RPC headers to use (e.g., "Key:Value")
        #[arg(long)]
        rpc_header: Option<Vec<String>>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let generator: Box<dyn WitnessGenerator> = match cli.source {
        SourceCommand::Tests { tag } => {
            todo!("call script");

            Box::new(ExecSpecTestBlocksAndWitnesses::new(
                directory_path,
                include.unwrap_or_default(),
                exclude.unwrap_or_default(),
            ))
        }
        SourceCommand::Rpc {
            last_n_blocks,
            block,
            rpc_url,
            rpc_header,
        } => {
            let parsed_headers: Vec<(String, String)> = rpc_header
                .unwrap_or_default()
                .into_iter()
                .map(|header| {
                    header
                        .split_once(':')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .ok_or_else(|| {
                            format!("invalid header format: '{}'. expected 'key:value'", header)
                        })
                })
                .collect::<Result<_, _>>()?;

            let mut builder =
                RPCBlocksAndWitnessesBuilder::new(rpc_url).with_headers(parsed_headers)?;

            if let Some(block_num) = block {
                builder = builder.block(block_num);
            } else {
                builder = builder.last_n_blocks(last_n_blocks.unwrap_or(1));
            }

            Box::new(builder.build()?)
        }
    };

    Ok(())
}
