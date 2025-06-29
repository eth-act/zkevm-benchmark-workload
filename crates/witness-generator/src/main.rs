use anyhow::anyhow;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use witness_generator::{
    WitnessGenerator,
    eest_generator::ExecSpecTestBlocksAndWitnessBuilder,
    rpc_generator::{RPCBlocksAndWitnessesBuilder, RpcFlatHeaderKeyValues},
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

        /// Include only test names with the provided strings.   
        #[arg(short, long)]
        include: Option<Vec<String>>,

        /// Exclude all test names with the provided strings.
        #[arg(short, long)]
        exclude: Option<Vec<String>>,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Generating fixtures in folder: {:?}", cli.output_folder);
    if !cli.output_folder.exists() {
        std::fs::create_dir_all(&cli.output_folder).map_err(|e| {
            anyhow!(
                "Failed to create output folder {:?}: {}",
                cli.output_folder,
                e
            )
        })?;
    }
    let generator: Box<dyn WitnessGenerator> = match cli.source {
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
                builder = builder.with_includes(include)
            }
            if let Some(exclude) = exclude {
                builder = builder.with_excludes(exclude)
            }
            Box::new(builder.build()?)
        }
        SourceCommand::Rpc {
            last_n_blocks,
            block,
            rpc_url,
            rpc_header,
        } => {
            let mut builder = RPCBlocksAndWitnessesBuilder::new(rpc_url);
            if let Some(rpc_header) = rpc_header {
                builder =
                    builder.with_headers(RpcFlatHeaderKeyValues::new(rpc_header).try_into()?)?;
            }
            if let Some(block_num) = block {
                builder = builder.block(block_num);
            } else {
                builder = builder.last_n_blocks(last_n_blocks.unwrap_or(1));
            }

            Box::new(builder.build()?)
        }
    };

    println!("Generating fixtures...");
    let bws = generator.generate().await?;
    println!("Generated {} blocks and witnesses", bws.len());

    if bws.is_empty() {
        println!("No blocks and witnesses generated. Exiting.");
        return Ok(());
    }
    println!("Writing fixtures to output folder...");
    for bw in bws {
        let output_path = cli.output_folder.join(format!("{}.json", bw.name));
        let output_data = serde_json::to_string(&bw)?;
        if !output_path.exists() {
            std::fs::create_dir_all(output_path.parent().unwrap())
                .map_err(|e| anyhow!("Failed to create directory for {:?}: {}", output_path, e))?;
        }
        std::fs::write(&output_path, output_data)
            .map_err(|e| anyhow!("Failed to write fixture to {:?}: {}", output_path, e))?;
    }
    println!("Fixtures written successfully.");

    Ok(())
}
