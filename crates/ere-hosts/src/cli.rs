//! CLI definitions for the zkVM benchmarker

use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use benchmark_runner::{runner::Action, stateless_validator};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::ProverResourceType;
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

    /// zkVM instances to benchmark (required for benchmark subcommands)
    #[arg(long, value_parser = <zkVMKind as std::str::FromStr>::from_str)]
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
    /// Ethereum Stateless Validator
    StatelessValidator {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,
        /// Execution client to benchmark
        #[arg(short, long)]
        execution_client: ExecutionClient,
        /// Block body DA mode (none to disable, raw, or snappy)
        #[arg(long, value_enum, default_value = "none")]
        block_body_da: BlockBodyDA,
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

    /// Analyze block body compression effectiveness for KZG blob reduction
    AnalyzeCompression {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,
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

/// Block body DA options for stateless validator
#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum BlockBodyDA {
    /// Disable block body DA
    #[default]
    None,
    /// Enable Block Body DA with raw body encoding
    Raw,
    /// Enable Block Body DA with Snappy-compressed body encoding
    Snappy,
}

impl BlockBodyDA {
    /// Convert to encoding and proof flag tuple.
    pub fn to_encoding_and_proof(self) -> (guest_libs::blobs::BlockBodyEncoding, bool) {
        match self {
            Self::None => (guest_libs::blobs::BlockBodyEncoding::Raw, false),
            Self::Raw => (guest_libs::blobs::BlockBodyEncoding::Raw, true),
            Self::Snappy => (guest_libs::blobs::BlockBodyEncoding::Snappy, true),
        }
    }
}

/// Execution clients for the stateless validator
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ExecutionClient {
    /// Reth execution client
    Reth,
    /// Ethrex execution client
    Ethrex,
}

impl ExecutionClient {
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

impl From<BlockEncodingFormat> for block_encoding_length_io::BlockEncodingFormat {
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
