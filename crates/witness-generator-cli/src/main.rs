//! CLI to generate fixtures for zkEVM benchmarking tool

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::EnvFilter;
use witness_generator::{
    WitnessGenerator,
    eest_generator::ExecSpecTestBlocksAndWitnessBuilder,
    rpc_generator::{RpcBlocksAndWitnessesBuilder, RpcFlatHeaderKeyValues},
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
    /// Generate fixtures from execution specification tests
    Tests {
        /// EEST release tag to use (e.g., "v0.1.0"). If empty, the latest release will be used.
        #[arg(short, long, conflicts_with = "eest_fixtures_path")]
        tag: Option<String>,

        /// Include only test names containing the provided strings.   
        #[arg(short, long)]
        include: Option<Vec<String>>,

        /// Exclude all test names containing the provided strings.
        #[arg(short, long)]
        exclude: Option<Vec<String>>,

        /// Optional input folder for EEST files. If not provided, the tag rule will be used.
        #[arg(long, conflicts_with = "tag")]
        eest_fixtures_path: Option<PathBuf>,
    },
    /// Generate fixtures from an RPC endpoint
    Rpc {
        /// Number of last blocks to pull
        #[arg(long, conflicts_with_all = ["block", "follow"])]
        last_n_blocks: Option<usize>,

        /// Specific block number to pull
        #[arg(long, conflicts_with_all = ["last_n_blocks", "follow"])]
        block: Option<u64>,

        /// Listen for new blocks
        #[arg(long, default_value_t = false, conflicts_with_all = ["last_n_blocks", "block"])]
        follow: bool,

        /// RPC URL to use (mandatory)
        #[arg(long)]
        rpc_url: String,

        /// Optional RPC headers to use (format: "Key:Value")
        #[arg(long)]
        rpc_header: Option<Vec<String>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let cli = Cli::parse();

    info!("Generating fixtures in folder: {:?}", cli.output_folder);
    if !cli.output_folder.exists() {
        std::fs::create_dir_all(&cli.output_folder)
            .with_context(|| format!("Failed to create output folder: {:?}", cli.output_folder))?;
    }

    let generator: Box<dyn WitnessGenerator> = build_generator(cli.source).await?;

    info!("Generating fixtures...");
    let count = generator
        .generate_to_path(&cli.output_folder)
        .await
        .context("Failed to generate blocks and witnesses")?;

    info!("Generated {} blocks and witnesses", count);

    Ok(())
}

async fn build_generator(source: SourceCommand) -> Result<Box<dyn WitnessGenerator>> {
    match source {
        SourceCommand::Tests {
            tag,
            include,
            exclude,
            eest_fixtures_path,
        } => {
            let mut builder = ExecSpecTestBlocksAndWitnessBuilder::default();

            if let Some(tag) = tag {
                builder = builder.with_tag(tag);
            } else if let Some(input_folder) = eest_fixtures_path {
                builder = builder.with_input_folder(input_folder)?;
            }

            if let Some(include) = include {
                builder = builder.with_includes(include);
            }
            if let Some(exclude) = exclude {
                builder = builder.with_excludes(exclude);
            }

            Ok(Box::new(
                builder.build().context("Failed to build EEST generator")?,
            ))
        }
        SourceCommand::Rpc {
            last_n_blocks,
            block,
            rpc_url,
            rpc_header,
            follow: listen,
        } => {
            let mut builder = RpcBlocksAndWitnessesBuilder::new(rpc_url);

            if let Some(rpc_header) = rpc_header {
                let headers = RpcFlatHeaderKeyValues::new(rpc_header)
                    .try_into()
                    .context("Failed to parse RPC headers")?;
                builder = builder.with_headers(headers);
            }

            if listen {
                let stop = CancellationToken::new();
                builder = builder.listen(stop.clone());

                tokio::spawn(async move {
                    tokio::select! {
                        _ = tokio::signal::ctrl_c() => {
                            info!("Stopping...");
                            stop.cancel();
                        }
                    }
                });
            } else if let Some(block_num) = block {
                builder = builder.block(block_num);
            } else {
                let n_blocks = last_n_blocks.unwrap_or(1);
                if n_blocks == 0 {
                    return Err(anyhow!("Number of blocks must be greater than 0"));
                }
                builder = builder.last_n_blocks(n_blocks);
            }

            Ok(Box::new(
                builder.build().context("Failed to build RPC generator")?,
            ))
        }
    }
}
