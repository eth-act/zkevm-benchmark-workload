//! CLI definitions for the zkVM benchmarker

use anyhow::{Result as AnyhowResult, bail};
use benchmark_runner::{runner::Action, stateless_validator};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::{ProverResource, RemoteProverConfig, zkVMKind};
use std::path::PathBuf;
use std::time::Duration;

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

    /// Endpoint URL of the proving cluster (required when --resource cluster)
    #[arg(long, required_if_eq("resource", "cluster"))]
    pub cluster_endpoint: Option<String>,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "execute")]
    pub action: BenchmarkAction,

    /// zkVM instances to benchmark
    #[arg(long, required(true), value_parser = <zkVMKind as std::str::FromStr>::from_str)]
    pub zkvms: Vec<zkVMKind>,

    /// Replace the selected action even if it already has a result
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

    /// Folder containing saved proofs (verify only; default: zkevm-fixtures-proofs)
    #[arg(long, conflicts_with = "proofs_url")]
    pub proofs_folder: Option<PathBuf>,

    /// URL to a .tar.gz archive containing proofs (used with --action verify).
    #[arg(long, conflicts_with = "proofs_folder")]
    pub proofs_url: Option<String>,

    /// Base path for pre-compiled guest program binaries. If not set, they will be downloaded
    /// from the resolved ere-guests release or commit artifacts.
    #[arg(long, conflicts_with = "guest_artifact_base_url")]
    pub bin_path: Option<PathBuf>,

    /// Base URL for pre-compiled guest program artifacts.
    #[arg(long, conflicts_with = "bin_path")]
    pub guest_artifact_base_url: Option<String>,

    /// Timeout for the selected action only, for example `15m`, `5m`, or `2s`.
    #[arg(long, value_name = "DURATION", value_parser = parse_duration)]
    pub timeout: Option<Duration>,

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
        /// EEST fixture file or folder (required for execute, estimate-cost, and prove; ignored for verify)
        #[arg(short, long)]
        input_folder: Option<PathBuf>,
        /// Fixture name prefix to run. Repeat to select multiple prefixes.
        #[arg(long, value_name = "PREFIX")]
        fixture: Option<Vec<String>>,
        /// Execution client to benchmark
        #[arg(short, long)]
        execution_client: ExecutionClient,
    },
}

/// Execution clients for the stateless validator
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ExecutionClient {
    /// Reth execution client
    Reth,
    /// Ethrex execution client
    Ethrex,
    /// Zesu execution client
    Zesu,
}

/// Prover resource types
#[derive(Debug, Clone, ValueEnum)]
pub enum Resource {
    /// CPU resource
    Cpu,
    /// GPU resource
    Gpu,
    /// Proving cluster (requires --cluster-endpoint)
    Cluster,
}

/// Benchmark actions
#[derive(Debug, Clone, ValueEnum)]
pub enum BenchmarkAction {
    /// Only do zkVM execution
    Execute,
    /// Execute once to estimate proving costs and heap use
    EstimateCost,
    /// Create a zkVM proof
    Prove,
    /// Verify proofs loaded from disk
    Verify,
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    humantime::parse_duration(value).map_err(|err| err.to_string())
}

impl Cli {
    /// Validate arguments whose requirements depend on the selected action.
    pub fn validate(&self) -> AnyhowResult<()> {
        let action = &self.action;
        if self.save_proofs.is_some() && !matches!(action, BenchmarkAction::Prove) {
            bail!("--save-proofs requires --action prove");
        }
        if (self.proofs_url.is_some() || self.proofs_folder.is_some())
            && !matches!(action, BenchmarkAction::Verify)
        {
            bail!("--proofs-folder and --proofs-url require --action verify");
        }
        if self.zisk_profile
            && (!matches!(action, BenchmarkAction::Execute) || self.zkvms != [zkVMKind::Zisk])
        {
            bail!("--zisk-profile requires --action execute and --zkvms zisk only");
        }
        if self.cluster_endpoint.is_some() && !matches!(self.resource, Resource::Cluster) {
            bail!("--cluster-endpoint requires --resource cluster");
        }
        if matches!(self.resource, Resource::Cluster) {
            if self.zkvms.iter().any(|zkvm| *zkvm != zkVMKind::Zisk) {
                bail!("--resource cluster requires --zkvms zisk only");
            }
            if matches!(
                action,
                BenchmarkAction::Execute | BenchmarkAction::EstimateCost
            ) {
                bail!("--resource cluster supports only --action prove or --action verify");
            }
        }
        let GuestProgramCommand::StatelessValidator {
            execution_client, ..
        } = &self.guest_program;
        let el: stateless_validator::ExecutionClient = (*execution_client).into();
        for zkvm in &self.zkvms {
            el.validate_zkvm(*zkvm)?;
        }
        if matches!(self.action, BenchmarkAction::Verify) {
            return Ok(());
        }

        let GuestProgramCommand::StatelessValidator { input_folder, .. } = &self.guest_program;
        let Some(input_folder) = input_folder else {
            bail!("--input-folder is required with --action execute, estimate-cost, or prove");
        };
        if !input_folder.exists() {
            bail!("input path does not exist: {}", input_folder.display());
        }

        Ok(())
    }

    /// Build the Ere [`ProverResource`] from parsed CLI args.
    pub fn prover_resource(&self) -> ProverResource {
        match self.resource {
            Resource::Cpu => ProverResource::Cpu,
            Resource::Gpu => ProverResource::Gpu,
            Resource::Cluster => ProverResource::Cluster(RemoteProverConfig {
                endpoint: self
                    .cluster_endpoint
                    .clone()
                    .expect("clap required_if_eq should guarantee cluster_endpoint set"),
                api_key: None,
            }),
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Self::Execute,
            BenchmarkAction::EstimateCost => Self::EstimateCost,
            BenchmarkAction::Prove => Self::Prove,
            BenchmarkAction::Verify => Self::Verify,
        }
    }
}

impl From<ExecutionClient> for stateless_validator::ExecutionClient {
    fn from(client: ExecutionClient) -> Self {
        match client {
            ExecutionClient::Reth => Self::Reth,
            ExecutionClient::Ethrex => Self::Ethrex,
            ExecutionClient::Zesu => Self::Zesu,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(action: &str, extra: &[&str]) -> AnyhowResult<Cli> {
        let mut args = vec!["ere-hosts", "--zkvms", "zisk", "--action", action];
        args.extend_from_slice(extra);
        args.extend([
            "stateless-validator",
            "--execution-client",
            "reth",
            "--input-folder",
            ".",
        ]);
        let cli = Cli::try_parse_from(args)?;
        cli.validate()?;
        Ok(cli)
    }

    #[test]
    fn estimation_rejects_proof_options_profiles_and_clusters() {
        assert!(parse("estimate-cost", &[]).is_ok());
        for args in [
            vec!["--save-proofs", "proofs"],
            vec!["--proofs-folder", "proofs"],
            vec!["--proofs-url", "https://example.com/proofs.tar.gz"],
            vec!["--zisk-profile"],
            vec![
                "--resource",
                "cluster",
                "--cluster-endpoint",
                "http://localhost:1234",
            ],
        ] {
            assert!(parse("estimate-cost", &args).is_err(), "{args:?}");
        }
        assert!(parse("prove", &["--save-proofs", "proofs"]).is_ok());
        assert!(parse("verify", &["--proofs-folder", "proofs"]).is_ok());
    }

    #[test]
    fn estimation_requires_input_and_zesu_requires_zisk() {
        let args = [
            "ere-hosts",
            "--zkvms",
            "zisk",
            "--action",
            "estimate-cost",
            "stateless-validator",
            "--execution-client",
            "zesu",
        ];
        assert!(Cli::try_parse_from(args).unwrap().validate().is_err());
        let mut with_input = args.to_vec();
        with_input.extend(["--input-folder", "."]);
        assert!(Cli::try_parse_from(&with_input).unwrap().validate().is_ok());
        with_input[2] = "sp1";
        assert!(
            Cli::try_parse_from(&with_input)
                .unwrap()
                .validate()
                .is_err()
        );
        with_input[2] = "zisk";
        *with_input.last_mut().unwrap() = "/path/that/does/not/exist";
        assert!(
            Cli::try_parse_from(&with_input)
                .unwrap()
                .validate()
                .is_err()
        );
        with_input[4] = "verify";
        assert!(Cli::try_parse_from(&with_input).unwrap().validate().is_ok());
    }
}
