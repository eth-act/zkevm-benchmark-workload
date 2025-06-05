//! Binary for benchmarking different Ere compatible zkVMs

use clap::{Parser, Subcommand, ValueEnum};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use witness_generator::{
    generate_stateless_witness::ExecSpecTestBlocksAndWitnesses, rpc::RPCBlocksAndWitnessesBuilder,
    witness_generator::WitnessGenerator,
};

use benchmark_runner::{Action, run_benchmark_ere};
use zkvm_interface::{Compiler, ProverResourceType};

#[cfg(feature = "sp1")]
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};

#[cfg(feature = "risc0")]
use ere_risczero::{EreRisc0, RV32_IM_RISCZERO_ZKVM_ELF};

#[cfg(feature = "openvm")]
use ere_openvm::{EreOpenVM, OPENVM_TARGET};

#[cfg(feature = "pico")]
use ere_pico::{ErePico, PICO_TARGET};

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

    /// Source of blocks and witnesses
    #[command(subcommand)]
    source: SourceCommand,
}

/// Constructs the absolute path to a subdirectory within the `zkevm-fixtures` submodule.
///
/// This is default tests directory path
fn path_to_zkevm_fixtures() -> &'static str {
    concat!(
        env!("CARGO_WORKSPACE_DIR"),
        "/zkevm-fixtures/fixtures/blockchain_tests"
    )
}

#[derive(Subcommand, Clone, Debug)]
enum SourceCommand {
    Tests {
        #[arg(short, long, default_value = path_to_zkevm_fixtures())]
        directory_path: PathBuf,
    },
    Mainnet {
        /// Number of last blocks to pull from mainnet (mandatory)
        #[arg(long)]
        last_n_blocks: usize,

        /// RPC URL to use (mandatory)
        #[arg(long)]
        rpc_url: String,

        /// Optional RPC headers to use (e.g., "Key:Value")
        #[arg(long)]
        rpc_header: Option<Vec<String>>,
    },
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
            Resource::Cpu => ProverResourceType::Cpu,
            Resource::Gpu => ProverResourceType::Gpu,
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Action::Execute,
            BenchmarkAction::Prove => Action::Prove,
        }
    }
}

/// Main entry point for the host benchmarker
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let resource: ProverResourceType = cli.resource.into();
    let action: Action = cli.action.into();

    let block_witness_gen: Box<dyn WitnessGenerator> = match cli.source {
        SourceCommand::Tests { directory_path } => {
            Box::new(ExecSpecTestBlocksAndWitnesses::new(directory_path))
        }
        SourceCommand::Mainnet {
            last_n_blocks,
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
            Box::new(
                RPCBlocksAndWitnessesBuilder::new(rpc_url)
                    .with_headers(parsed_headers)?
                    .last_n_blocks(last_n_blocks)
                    .build()?,
            )
        }
    };

    // Set to true once a zkvm has ran
    let mut ran_any = false;

    #[cfg(feature = "sp1")]
    {
        run_cargo_patch_command("sp1")?;
        let sp1_zkvm = new_sp1_zkvm(resource)?;
        run_benchmark_ere("sp1", sp1_zkvm, action, &block_witness_gen).await?;
        ran_any = true;
    }

    #[cfg(feature = "risc0")]
    {
        run_cargo_patch_command("risc0")?;
        let risc0_zkvm = new_risczero_zkvm(resource)?;
        run_benchmark_ere("risc0", risc0_zkvm, action, &block_witness_gen).await?;
        ran_any = true;
    }

    #[cfg(feature = "openvm")]
    {
        run_cargo_patch_command("openvm")?;
        let openvm_zkvm = new_openvm_zkvm(resource)?;
        run_benchmark_ere("openvm", openvm_zkvm, action, &block_witness_gen).await?;
        ran_any = true;
    }

    #[cfg(feature = "pico")]
    {
        let pico_zkvm = new_pico_zkvm(resource)?;
        run_benchmark_ere("pico", pico_zkvm, action, &block_witness_gen).await?;
        ran_any = true;
    }

    if ran_any {
        Ok(())
    } else {
        Err("please enable one of the zkVM's using the appropriate feature flag".into())
    }
}

#[cfg(feature = "sp1")]
fn new_sp1_zkvm(prover_resource: ProverResourceType) -> Result<EreSP1, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/sp1");
    let program = RV32_IM_SUCCINCT_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreSP1::new(program, prover_resource))
}

#[cfg(feature = "risc0")]
fn new_risczero_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreRisc0, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/risc0");
    let program = RV32_IM_RISCZERO_ZKVM_ELF::compile(&PathBuf::from(guest_dir))?;
    Ok(EreRisc0::new(program, prover_resource))
}

#[cfg(feature = "openvm")]
fn new_openvm_zkvm(
    prover_resource: ProverResourceType,
) -> Result<EreOpenVM, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/openvm");
    let program = OPENVM_TARGET::compile(&PathBuf::from(guest_dir))?;
    Ok(EreOpenVM::new(program, prover_resource))
}

#[cfg(feature = "pico")]
fn new_pico_zkvm(
    prover_resource: ProverResourceType,
) -> Result<ErePico, Box<dyn std::error::Error>> {
    let guest_dir = concat!(env!("CARGO_WORKSPACE_DIR"), "ere-guests/pico");
    let program = PICO_TARGET::compile(&PathBuf::from(guest_dir))?;
    Ok(ErePico::new(program, prover_resource))
}

/// Patches the precompiles for a specific zkvm
fn run_cargo_patch_command(zkvm_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running cargo {}...", zkvm_name);

    let output = Command::new("cargo").arg(zkvm_name).output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        eprintln!(
            "cargo {} failed with exit code: {:?}",
            zkvm_name,
            output.status.code()
        );
        eprintln!("stdout: {}", stdout);
        eprintln!("stderr: {}", stderr);

        return Err(format!("cargo {} command failed", zkvm_name).into());
    }

    println!("cargo {} completed successfully", zkvm_name);
    Ok(())
}
