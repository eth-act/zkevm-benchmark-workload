use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
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
        #[arg(short, long)]
        tag: Option<String>,

        /// Include only test names containing the provided strings.   
        #[arg(short, long)]
        include: Option<Vec<String>>,

        /// Exclude all test names containing the provided strings.
        #[arg(short, long)]
        exclude: Option<Vec<String>>,
    },
    /// Generate fixtures from an RPC endpoint
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

        /// Optional RPC headers to use (format: "Key:Value")
        #[arg(long)]
        rpc_header: Option<Vec<String>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("Generating fixtures in folder: {:?}", cli.output_folder);
    if !cli.output_folder.exists() {
        std::fs::create_dir_all(&cli.output_folder)
            .with_context(|| format!("Failed to create output folder: {:?}", cli.output_folder))?;
    }

    let generator: Box<dyn WitnessGenerator> = build_generator(cli.source).await?;

    println!("Generating fixtures...");
    let bws = generator
        .generate()
        .await
        .context("Failed to generate blocks and witnesses")?;

    println!("Generated {} blocks and witnesses", bws.len());

    if bws.is_empty() {
        println!("No blocks and witnesses generated. Exiting.");
        return Ok(());
    }

    write_fixtures_to_disk(&cli.output_folder, &bws).context("Failed to write fixtures to disk")?;

    println!("Fixtures written successfully.");
    Ok(())
}

async fn build_generator(source: SourceCommand) -> Result<Box<dyn WitnessGenerator>> {
    match source {
        SourceCommand::Tests {
            tag,
            include,
            exclude,
        } => {
            let mut builder = ExecSpecTestBlocksAndWitnessBuilder::new();

            if let Some(tag) = tag {
                builder = builder.with_tag(tag);
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
        } => {
            let mut builder = RpcBlocksAndWitnessesBuilder::new(rpc_url);

            if let Some(rpc_header) = rpc_header {
                let headers = RpcFlatHeaderKeyValues::new(rpc_header)
                    .try_into()
                    .context("Failed to parse RPC headers")?;
                builder = builder.with_headers(headers);
            }

            if let Some(block_num) = block {
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

fn write_fixtures_to_disk(
    output_folder: &Path,
    bws: &[witness_generator::BlocksAndWitnesses],
) -> Result<()> {
    println!("Writing fixtures to output folder...");

    for bw in bws {
        let output_path = output_folder.join(format!("{}.json", bw.name));
        let output_data = serde_json::to_string_pretty(bw)
            .with_context(|| format!("Failed to serialize fixture: {}", bw.name))?;

        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory for: {output_path:?}"))?;
        }

        std::fs::write(&output_path, output_data)
            .with_context(|| format!("Failed to write fixture to: {output_path:?}"))?;
    }

    Ok(())
}
