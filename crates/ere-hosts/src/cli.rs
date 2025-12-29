//! CLI definitions for the zkVM benchmarker

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use benchmark_runner::{stateless_executor, stateless_validator};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::{NetworkProverConfig, ProverResourceType};

/// Command line interface for the zkVM benchmarker
#[derive(Parser)]
#[command(name = "zkvm-benchmarker")]
#[command(about = "Benchmark different Ere compatible zkVMs")]
#[command(version)]
#[derive(Debug)]
pub struct Cli {
    /// Resource type for proving
    #[arg(short, long, value_enum, default_value = "cpu")]
    pub resource: Resource,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "execute")]
    pub action: BenchmarkAction,

    /// zkVM instances to benchmark
    #[arg(long, required(true), value_parser = <zkVMKind as std::str::FromStr>::from_str)]
    pub zkvms: Vec<zkVMKind>,

    /// Rerun the benchmarks even if the output folder already contains results
    #[arg(long, default_value_t = false)]
    pub force_rerun: bool,

    /// Guest program to benchmark
    #[command(subcommand)]
    pub guest_program: GuestProgramCommand,

    /// Output folder for benchmark results
    #[arg(short, long, default_value = "zkevm-metrics")]
    pub output_folder: PathBuf,

    /// Output folder for dumping input files used in benchmarks
    #[arg(long)]
    pub dump_inputs: Option<PathBuf>,
}

/// Subcommands for different guest programs
#[derive(Subcommand, Clone, Debug)]
pub enum GuestProgramCommand {
    /// Ethereum Stateless Executor
    StatelessExecutor {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input", conflicts_with = "input_file")]
        input_folder: PathBuf,
        /// Input file for a single benchmark fixture
        #[arg(long)]
        input_file: Option<PathBuf>,
        /// Execution client to benchmark
        #[arg(short, long, value_enum, default_value = "reth")]
        execution_client: StatelessExecutorClient,
    },
    /// Ethereum Stateless Validator
    StatelessValidator {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input", conflicts_with = "input_file")]
        input_folder: PathBuf,
        /// Input file for a single benchmark fixture
        #[arg(long)]
        input_file: Option<PathBuf>,
        /// Execution client to benchmark
        #[arg(short, long)]
        execution_client: StatelessValidatorClient,
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

/// Encoding formats for block encoding length program
#[derive(Debug, Clone, ValueEnum)]
pub enum BlockEncodingFormat {
    /// RLP encoding
    Rlp,
    /// SSZ encoding
    Ssz,
}

/// Execution clients for the stateless executor
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum StatelessExecutorClient {
    /// Reth execution client
    Reth,
}

impl StatelessExecutorClient {
    /// Get the guest relative path for the execution client
    pub fn guest_rel_path(&self) -> Result<PathBuf> {
        let path = match self {
            Self::Reth => "stateless-executor",
        };
        Ok(PathBuf::from_str(path).unwrap())
    }
}

/// Execution clients for the stateless validator
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum StatelessValidatorClient {
    /// Reth execution client
    Reth,
    /// Ethrex execution client
    Ethrex,
}

impl StatelessValidatorClient {
    /// Get the guest relative path for the execution client
    pub fn guest_rel_path(&self) -> Result<PathBuf> {
        let path = match self {
            Self::Reth => "stateless-validator/reth",
            Self::Ethrex => "stateless-validator/ethrex",
        };
        Ok(PathBuf::from_str(path).unwrap())
    }
}

/// Prover resource types
#[derive(Debug, Clone, ValueEnum)]
pub enum Resource {
    /// CPU resource
    Cpu,
    /// GPU resource
    Gpu,
    /// Network resource (SP1 only, NETWORK_PRIVATE_KEY env var is optional)
    Network,
}

/// Benchmark actions
#[derive(Debug, Clone, ValueEnum)]
pub enum BenchmarkAction {
    /// Only do zkVM execution
    Execute,
    /// Create a zkVM proof
    Prove,
}

impl From<Resource> for ProverResourceType {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::Cpu => Self::Cpu,
            Resource::Gpu => Self::Gpu,
            Resource::Network => Self::Network(NetworkProverConfig::default()),
        }
    }
}

impl From<BenchmarkAction> for benchmark_runner::runner::Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Self::Execute,
            BenchmarkAction::Prove => Self::Prove,
        }
    }
}

impl From<BlockEncodingFormat> for block_encoding_length_guest::guest::BlockEncodingFormat {
    fn from(format: BlockEncodingFormat) -> Self {
        match format {
            BlockEncodingFormat::Rlp => Self::Rlp,
            BlockEncodingFormat::Ssz => Self::Ssz,
        }
    }
}

impl From<StatelessExecutorClient> for stateless_executor::ExecutionClient {
    fn from(client: StatelessExecutorClient) -> Self {
        match client {
            StatelessExecutorClient::Reth => Self::Reth,
        }
    }
}

impl From<StatelessValidatorClient> for stateless_validator::ExecutionClient {
    fn from(client: StatelessValidatorClient) -> Self {
        match client {
            StatelessValidatorClient::Reth => Self::Reth,
            StatelessValidatorClient::Ethrex => Self::Ethrex,
        }
    }
}
