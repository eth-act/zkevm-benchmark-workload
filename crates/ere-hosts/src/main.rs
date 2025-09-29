//! Binary for benchmarking different Ere compatible zkVMs

#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use benchmark_runner::{
    block_encoding_length_program, empty_program,
    runner::{Action, RunConfig, get_zkvm_instances, run_benchmark},
    stateless_validator::{self},
    trie_bench
};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::ErezkVM;
use std::path::{Path, PathBuf};
use tracing::info;
use tracing_subscriber::EnvFilter;
use zkvm_interface::ProverResourceType;

#[derive(Parser)]
#[command(name = "zkvm-benchmarker")]
#[command(about = "Benchmark different Ere compatible zkVMs")]
#[command(version)]
struct Cli {
    /// Resource type for proving
    #[arg(short, long, value_enum, default_value = "cpu")]
    resource: Resource,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "execute")]
    action: BenchmarkAction,

    /// zkVM instances to benchmark
    #[arg(long, required(true), value_parser = <ErezkVM as std::str::FromStr>::from_str)]
    zkvms: Vec<ErezkVM>,

    /// Rerun the benchmarks even if the output folder already contains results
    #[arg(long, default_value_t = false)]
    force_rerun: bool,

    /// Guest program to benchmark
    #[command(subcommand)]
    guest_program: GuestProgramCommand,

    /// Output folder for benchmark results
    #[arg(short, long, default_value = "zkevm-metrics")]
    output_folder: PathBuf,
}

#[derive(Subcommand, Clone, Debug)]
enum GuestProgramCommand {
    /// Ethereum Stateless Validator
    StatelessValidator {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,
        #[arg(short, long)]
        execution_client: ExecutionClient,
    },
    /// Trie benchmark
    TrieBenchmark {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,
        #[arg(short, long)]
        execution_client: ExecutionClient,
    },
    /// Empty program
    EmptyProgram,

    /// Block encoding length
    BlockEncodingLength {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,

        /// Number of times to loop the benchmark
        #[arg(long)]
        loop_count: u16,

        /// Encoding format
        #[arg(short, long, value_enum)]
        format: BlockEncodingFormat,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum BlockEncodingFormat {
    Rlp,
    Ssz,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum ExecutionClient {
    Reth,
    Ethrex,
}

impl ExecutionClient {
    const fn guest_rel_path(&self) -> &str {
        match self {
            Self::Reth => "stateless-validator/reth",
            Self::Ethrex => "stateless-validator/ethrex",
        }
    }
}

#[derive(Clone, ValueEnum)]
enum Resource {
    Cpu,
    Gpu,
}

#[derive(Clone, ValueEnum)]
enum BenchmarkAction {
    Execute,
    Prove,
}

impl From<Resource> for ProverResourceType {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Cpu => Self::Cpu,
            Resource::Gpu => Self::Gpu,
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Self::Execute,
            BenchmarkAction::Prove => Self::Prove,
        }
    }
}

impl From<BlockEncodingFormat> for block_encoding_length_program::BlockEncodingFormat {
    fn from(format: BlockEncodingFormat) -> Self {
        match format {
            BlockEncodingFormat::Rlp => Self::Rlp,
            BlockEncodingFormat::Ssz => Self::Ssz,
        }
    }
}

impl From<ExecutionClient> for stateless_validator::ExecutionClient {
    fn from(client: ExecutionClient) -> Self {
        match client {
            ExecutionClient::Reth => Self::Reth,
            ExecutionClient::Ethrex => Self::Ethrex,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();
    info!(
        "Running benchmarks with resource={:?} and action={:?}",
        resource, action
    );

    let config = RunConfig {
        output_folder: cli.output_folder,
        action,
        force_rerun: cli.force_rerun,
    };

    let workspace_dir = workspace_root().join("ere-guests");
    match &cli.guest_program {
        GuestProgramCommand::StatelessValidator {
            input_folder,
            execution_client,
        } => {
            info!(
                "Running stateless-validator benchmark for input folder: {}",
                input_folder.display()
            );
            let guest_io = stateless_validator::stateless_validator_inputs(
                input_folder.as_path(),
                (*execution_client).into(),
            )?;
            let guest_relative = Path::new(execution_client.guest_rel_path());
            let apply_patches = matches!(execution_client, ExecutionClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                guest_relative,
                resource,
                apply_patches,
            )?;
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, guest_io.clone())?;
            }
        }
        GuestProgramCommand::EmptyProgram => {
            info!("Running empty-program benchmarks");
            let guest_io = empty_program::empty_program_input();
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("empty-program"),
                resource,
                true,
            )?;
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, vec![guest_io.clone()])?;
            }
        }
        GuestProgramCommand::BlockEncodingLength {
            input_folder,
            loop_count,
            format,
        } => {
            info!(
                "Running {:?}-encoding-length benchmarks for input folder {} and loop count {}",
                format,
                input_folder.display(),
                loop_count
            );
            let guest_io = block_encoding_length_program::block_encoding_length_inputs(
                input_folder.as_path(),
                *loop_count,
                format.clone().into(),
            )?;
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("block-encoding-length"),
                resource,
                true,
            )?;
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, guest_io.clone())?;
            }
        }
        GuestProgramCommand::TrieBenchmark {
            input_folder,
            execution_client,
        } => {
            info!(
                "Running trie benchmark for input folder: {}",
                input_folder.display()
            );
            let guest_io = trie_bench::trie_bench_inputs(
                input_folder.as_path()
            )?;
            let apply_patches = matches!(execution_client, ExecutionClient::Reth);
            let zkvms = get_zkvm_instances(
                &cli.zkvms,
                &workspace_dir,
                Path::new("trie-bench"),
                resource,
                apply_patches,
            )?;
            for zkvm in zkvms {
                run_benchmark(&zkvm, &config, guest_io.clone())?;
            }
        }
    }

    Ok(())
}

/// Repository root (assumes `ere-hosts` lives in `<root>/crates/ere-hosts`).
fn workspace_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p
}
