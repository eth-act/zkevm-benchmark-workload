//! CLI definitions for the zkVM benchmarker

use benchmark_runner::{runner::Action, stateless_validator};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::ProverResourceType;
use std::path::PathBuf;

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

    /// Save generated proofs to the specified folder (only valid with --action prove)
    #[arg(long)]
    pub save_proofs: Option<PathBuf>,

    /// Folder containing saved proofs (used with --action verify)
    #[arg(
        long,
        default_value = "zkevm-fixtures-proofs",
        conflicts_with = "proofs_url"
    )]
    pub proofs_folder: PathBuf,

    /// URL to a .tar.gz archive containing proofs (used with --action verify).
    #[arg(long, conflicts_with = "proofs_folder")]
    pub proofs_url: Option<String>,

    /// Base path for pre-compiled guest program binaries. If not set, they will be downloaded
    /// from the latest ere-guests release.
    #[arg(long)]
    pub bin_path: Option<PathBuf>,

    /// Number of full warmup passes before measured verification (used with --action verify)
    #[arg(long, default_value_t = 3)]
    pub warmup_rounds: u32,

    /// Enable Zisk profiling (requires --zkvms zisk, --action execute)
    #[arg(long)]
    pub zisk_profile: bool,

    /// Output folder for Zisk profile results
    #[arg(long, default_value = "zisk-profiles")]
    pub zisk_profile_output: PathBuf,
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
    pub fn guest_rel_path(&self) -> PathBuf {
        let path = match self {
            Self::Reth => "stateless-validator/reth",
            Self::Ethrex => "stateless-validator/ethrex",
        };
        PathBuf::from(path)
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
    /// Verify proofs loaded from disk
    Verify,
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
            BenchmarkAction::Verify => Self::Verify,
        }
    }
}

impl From<BlockEncodingFormat> for ere_guests_block_encoding_length::guest::BlockEncodingFormat {
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
