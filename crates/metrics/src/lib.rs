#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub use chrono;

use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, io::Write, path::Path, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use thiserror::Error;

/// Represents a single benchmark run.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BenchmarkRun<Metadata> {
    /// Name of the benchmark.
    pub name: String,
    /// Timestamp when the benchmark run ended.
    pub timestamp_completed: chrono::DateTime<chrono::Utc>,
    /// Metadata
    pub metadata: Metadata,
    /// Execution metrics for the benchmark run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionMetrics>,
    /// Proving metrics for the benchmark run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proving: Option<ProvingMetrics>,
    /// Standalone verification metrics for the benchmark run.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub verification: Option<VerificationMetrics>,
    /// Cost estimation results for the benchmark run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_estimation: Option<CostEstimationMetrics>,
}

/// Hardware specs of the benchmark runner.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HardwareInfo {
    /// CPU model name.
    pub cpu_model: String,
    /// Total RAM in GiB.
    pub total_ram_gib: u64,
    /// Available GPUs.
    pub gpus: Vec<GpuInfo>,
}

/// Information about a GPU.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GpuInfo {
    /// GPU model name.
    pub model: String,
}

impl HardwareInfo {
    /// Detects hardware information from the current system.
    pub fn detect() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        Self {
            cpu_model: system
                .cpus()
                .first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string()),
            total_ram_gib: system.total_memory() / (1024 * 1024 * 1024),
            gpus: detect_gpus(),
        }
    }

    /// Serializes the hardware information to a JSON string in the provided path.
    pub fn to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), MetricsError> {
        let path = path.as_ref();
        ensure_parent_dirs(path)?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

/// Detects available GPUs on the system.
fn detect_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();

    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=gpu_name")
        .arg("--format=csv,noheader,nounits")
        .output()
        && output.status.success()
    {
        let gpu_names = String::from_utf8_lossy(&output.stdout);
        for line in gpu_names.lines() {
            let gpu_name = line.trim();
            if !gpu_name.is_empty() {
                gpus.push(GpuInfo {
                    model: gpu_name.to_string(),
                });
            }
        }
    }

    gpus
}

/// Information about a crash that occurred during a workload.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CrashInfo {
    /// The reason for the crash (e.g., panic message).
    pub reason: String,
}

/// Metrics for execution workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMetrics {
    /// Metrics for a successful execution workload.
    Success {
        /// Whether public output matched the fixture's expected public values.
        output_matched: bool,
        /// Execution duration.
        execution_duration: Duration,
    },
    /// Metrics for a crashed execution workload.
    Crashed(CrashInfo),
}

/// Metrics for proving workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProvingMetrics {
    /// Metrics for a successful proving workload.
    Success {
        /// Whether prover and verification public outputs matched the fixture's expected public values.
        output_matched: bool,
        /// Proof size in bytes.
        proof_size: usize,
        /// Proving time in milliseconds.
        proving_time_ms: u128,
        /// Verification time in milliseconds.
        verification_time_ms: u128,
    },
    /// Metrics for a crashed proving workload.
    Crashed(CrashInfo),
}

/// Metrics for standalone verification workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMetrics {
    /// Metrics for a successful verification workload.
    Success {
        /// Proof size in bytes.
        proof_size: usize,
        /// Verification time in milliseconds.
        verification_time_ms: u128,
    },
    /// Metrics for a crashed verification workload.
    Crashed(CrashInfo),
}

/// Identity and configuration of a cost estimate.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CostEstimationContext {
    /// Execution client name.
    pub execution_client: String,
    /// Version reported by the guest catalog.
    pub execution_client_version: String,
    /// zkVM name.
    pub zkvm: String,
    /// zkVM SDK version.
    pub sdk_version: String,
    /// Ere revision used by the Docker image.
    pub ere_revision: String,
    /// SHA-256 of the actual guest ELF, including custom artifacts.
    pub elf_sha256: String,
    /// SHA-256 of the raw guest input.
    pub input_sha256: String,
    /// Effective settings that affect cost or heap estimation.
    pub estimator_settings: BTreeMap<String, String>,
}

/// Estimated proving cost, in units defined by each zkVM.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CostEstimationMetrics {
    /// A completed estimate; output mismatches remain visible.
    Success {
        /// Whether the public values matched the fixture.
        output_matched: bool,
        /// Cost per component, using the upstream component names.
        cost: BTreeMap<String, u64>,
        /// Estimated heap use in bytes, or null if unavailable.
        peak_heap_bytes: Option<u64>,
        /// Information needed to compare compatible estimates.
        context: Box<CostEstimationContext>,
    },
    /// An estimator error or panic.
    Crashed(CrashInfo),
}

/// Errors that can occur during metrics processing.
#[derive(Error, Debug)]
pub enum MetricsError {
    /// The existing record cannot be safely updated.
    #[error("cannot merge metrics: {0}")]
    InvalidUpdate(String),
    /// Error during JSON serialization or deserialization.
    #[error("serde (de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Error during file system I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl MetricsError {
    #[cfg(test)]
    fn into_serde_err(self) -> serde_json::Error {
        match self {
            Self::Serde(e) => e,
            other => panic!("unexpected error in test: {other}"),
        }
    }
}

impl<Metadata: serde::Serialize + serde::de::DeserializeOwned> BenchmarkRun<Metadata> {
    /// Serializes a list of `BenchmarkRun<Metadata>` into a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if serialization fails.
    pub fn to_json(items: &[Self]) -> Result<String, MetricsError> {
        serde_json::to_string(items).map_err(MetricsError::from)
    }

    /// Deserializes a list of `BenchmarkRun<Metadata>` from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if deserialization fails.
    pub fn from_json(json: &str) -> Result<Vec<Self>, MetricsError> {
        serde_json::from_str(json).map_err(MetricsError::from)
    }

    /// Serializes using JSON pretty-print and writes them to `path` atomically.
    ///
    /// The file is created if it does not exist and truncated if it does.
    /// Parent directories are created if they are missing.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if any filesystem operation fails.
    /// Returns `MetricsError::Serde` if JSON serialization fails.
    pub fn to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), MetricsError> {
        let path = path.as_ref();
        write_json_atomically(path, &serde_json::to_value(self)?)
    }

    /// Merges populated actions into a fixture file and replaces it atomically.
    /// Existing action payloads and non-null metadata remain intact.
    /// Callers must serialize updates to the same file across processes.
    ///
    /// # Errors
    /// Returns an error for invalid existing JSON, a different fixture name, or I/O failure.
    pub fn merge_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), MetricsError> {
        let path = path.as_ref();
        let mut update = serde_json::to_value(self)?;
        match fs::read(path) {
            Ok(bytes) => {
                let mut existing: serde_json::Value = serde_json::from_slice(&bytes)?;
                // Validate known payloads but keep their original JSON, including unknown fields.
                let previous: BenchmarkRun<serde_json::Value> =
                    serde_json::from_value(existing.clone())?;
                if previous.name != self.name {
                    return Err(MetricsError::InvalidUpdate(format!(
                        "{} contains fixture {:?}, expected {:?}",
                        path.display(),
                        previous.name,
                        self.name
                    )));
                }
                for field in ["execution", "proving", "verification", "cost_estimation"] {
                    if let Some(value) = update.get(field) {
                        existing[field] = value.clone();
                    }
                }
                if existing["metadata"].is_null() {
                    existing["metadata"] = update["metadata"].take();
                }
                existing["timestamp_completed"] = update["timestamp_completed"].take();
                update = existing;
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(err.into()),
        }
        write_json_atomically(path, &update)
    }

    /// Reads the file at `path` and deserializes a `BenchmarkRun<Metadata>` from its JSON content.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if reading the file fails.
    /// Returns `MetricsError::Serde` if JSON deserialization fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, MetricsError> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

fn write_json_atomically(path: &Path, value: &serde_json::Value) -> Result<(), MetricsError> {
    ensure_parent_dirs(path)?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    file.write_all(b"\n")?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|err| err.error)?;
    Ok(())
}

fn ensure_parent_dirs<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct Metadata {
        block_gas_used: u64,
    }

    // This is just a fixed sample we are using to test serde_roundtrip
    fn sample() -> Vec<BenchmarkRun<Metadata>> {
        vec![
            BenchmarkRun {
                name: "fft_bench".into(),
                timestamp_completed: chrono::Utc::now(),
                metadata: Metadata {
                    block_gas_used: 12345,
                },
                execution: Some(ExecutionMetrics::Success {
                    output_matched: true,

                    execution_duration: Duration::from_millis(150),
                }),
                proving: None,
                verification: None,
                cost_estimation: None,
            },
            BenchmarkRun {
                name: "aes_bench".into(),
                timestamp_completed: chrono::Utc::now(),
                metadata: Metadata {
                    block_gas_used: 67890,
                },
                execution: Some(ExecutionMetrics::Success {
                    output_matched: true,

                    execution_duration: Duration::from_millis(300),
                }),
                proving: Some(ProvingMetrics::Success {
                    output_matched: true,
                    proof_size: 256,
                    proving_time_ms: 2_000,
                    verification_time_ms: 200,
                }),
                verification: None,
                cost_estimation: None,
            },
            BenchmarkRun {
                name: "proving_bench".into(),
                timestamp_completed: chrono::Utc::now(),
                metadata: Metadata {
                    block_gas_used: 54321,
                },
                execution: None,
                proving: Some(ProvingMetrics::Success {
                    output_matched: true,
                    proof_size: 512,
                    proving_time_ms: 5_000,
                    verification_time_ms: 500,
                }),
                verification: None,
                cost_estimation: None,
            },
        ]
    }

    #[test]
    fn round_trip_json() {
        let runs = sample();
        let json = BenchmarkRun::to_json(&runs).expect("serialize");
        let parsed = BenchmarkRun::from_json(&json).expect("deserialize");
        assert_eq!(runs, parsed);
    }

    #[test]
    fn bad_json_is_error() {
        let bad = "{this is not valid json}";
        let err = BenchmarkRun::<()>::from_json(bad).unwrap_err();
        assert!(err.into_serde_err().is_data());
    }

    #[test]
    fn file_round_trip() -> Result<(), MetricsError> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();
        for run in sample() {
            run.to_path(path)?;
            let read_back = BenchmarkRun::from_path(path)?;
            assert_eq!(run, read_back);
        }

        Ok(())
    }

    #[test]
    fn test_name_accessor() {
        let benchmark_run = BenchmarkRun {
            name: "test_benchmark".into(),
            timestamp_completed: chrono::Utc::now(),
            metadata: Metadata {
                block_gas_used: 11111,
            },
            execution: Some(ExecutionMetrics::Success {
                output_matched: true,

                execution_duration: Duration::from_millis(150),
            }),
            proving: None,
            verification: None,
            cost_estimation: None,
        };

        assert_eq!(benchmark_run.name, "test_benchmark");
    }

    #[test]
    fn test_mixed_metrics_serialization() {
        let bench = BenchmarkRun {
            name: "mixed_bench".into(),
            timestamp_completed: chrono::Utc::now(),
            metadata: Metadata {
                block_gas_used: 22222,
            },
            execution: Some(ExecutionMetrics::Success {
                output_matched: true,

                execution_duration: Duration::from_millis(100),
            }),
            proving: Some(ProvingMetrics::Success {
                output_matched: true,
                proof_size: 128,
                proving_time_ms: 1500,
                verification_time_ms: 150,
            }),
            verification: None,
            cost_estimation: None,
        };
        let json = BenchmarkRun::to_json(std::slice::from_ref(&bench)).expect("serialize mixed");
        let parsed = BenchmarkRun::from_json(&json).expect("deserialize mixed");
        assert_eq!(vec![bench], parsed);
    }

    #[test]
    fn execution_success_serializes_output_matched_values() {
        for output_matched in [true, false] {
            let metrics = ExecutionMetrics::Success {
                output_matched,

                execution_duration: Duration::from_millis(7),
            };

            let value = serde_json::to_value(&metrics).expect("serialize execution metrics");
            assert_eq!(
                value["success"]["output_matched"],
                serde_json::Value::Bool(output_matched)
            );

            let parsed: ExecutionMetrics =
                serde_json::from_value(value).expect("deserialize execution metrics");
            assert_eq!(metrics, parsed);
        }
    }

    #[test]
    fn proving_success_serializes_output_matched_values() {
        for output_matched in [true, false] {
            let metrics = ProvingMetrics::Success {
                output_matched,
                proof_size: 256,
                proving_time_ms: 2_000,
                verification_time_ms: 200,
            };

            let value = serde_json::to_value(&metrics).expect("serialize proving metrics");
            assert_eq!(
                value["success"]["output_matched"],
                serde_json::Value::Bool(output_matched)
            );

            let parsed: ProvingMetrics =
                serde_json::from_value(value).expect("deserialize proving metrics");
            assert_eq!(metrics, parsed);
        }
    }

    fn action_run(field: &str, payload: serde_json::Value) -> BenchmarkRun<serde_json::Value> {
        serde_json::from_value(serde_json::json!({
            "name": "fixture",
            "timestamp_completed": "2026-09-10T00:00:00Z",
            "metadata": null,
            (field): payload,
        }))
        .unwrap()
    }

    fn estimated(heap: Option<u64>) -> serde_json::Value {
        serde_json::json!({"success": {
            "output_matched": false,
            "cost": {"opcode": u64::MAX, "system": 0},
            "peak_heap_bytes": heap,
            "context": {
                "execution_client": "reth", "execution_client_version": "0.1.0-rc.3",
                "zkvm": "sp1", "sdk_version": "v6.4.0", "ere_revision": "5023513",
                "elf_sha256": "elf", "input_sha256": "input",
                "estimator_settings": {"heap_start": "_end"}
            }
        }})
    }

    #[test]
    fn costs_round_trip_without_losing_large_integers_or_null_heap() {
        for heap in [None, Some(0), Some(1234)] {
            let run = action_run("cost_estimation", estimated(heap));
            let value = serde_json::to_value(&run).unwrap();
            assert_eq!(value["cost_estimation"], estimated(heap));
            assert_eq!(
                serde_json::from_value::<BenchmarkRun<serde_json::Value>>(value).unwrap(),
                run
            );
        }
    }

    #[test]
    fn merge_actions_in_any_order_and_replace_only_the_selected_action() -> Result<(), MetricsError>
    {
        let dir = tempfile::tempdir()?;
        let actions = [
            action_run(
                "execution",
                serde_json::json!({"success": {
                    "output_matched": true, "execution_duration": {"secs": 1, "nanos": 123}
                }}),
            ),
            action_run(
                "proving",
                serde_json::json!({"success": {
                    "output_matched": true, "proof_size": 42, "proving_time_ms": 1234, "verification_time_ms": 5
                }}),
            ),
            action_run(
                "verification",
                serde_json::json!({"success": {"proof_size": 42, "verification_time_ms": 7}}),
            ),
            action_run("cost_estimation", estimated(None)),
        ];
        for order in [[0, 1, 2, 3], [3, 2, 1, 0], [2, 0, 3, 1]] {
            let path = dir.path().join(format!("{order:?}.json"));
            for index in order {
                let mut update = actions[index].clone();
                if index != 2 {
                    update.metadata =
                        serde_json::json!({"original_test_name": "test", "block_index": 0});
                }
                update.merge_to_path(&path)?;
            }
            let before: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
            assert_eq!(before["metadata"]["original_test_name"], "test");
            for field in ["execution", "proving", "verification", "cost_estimation"] {
                assert!(!before[field].is_null());
            }
            let mut replacement = action_run(
                "execution",
                serde_json::json!({"crashed": {"reason": "new failure"}}),
            );
            replacement.timestamp_completed += chrono::Duration::seconds(1);
            replacement.merge_to_path(&path)?;
            let after: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
            for field in ["metadata", "proving", "verification", "cost_estimation"] {
                assert_eq!(before[field], after[field]);
            }
            assert_eq!(after["execution"]["crashed"]["reason"], "new failure");
            assert_ne!(before["timestamp_completed"], after["timestamp_completed"]);
        }
        Ok(())
    }

    #[test]
    fn merge_preserves_unknown_payload_fields_and_rejects_invalid_records()
    -> Result<(), MetricsError> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("fixture.json");
        let update = action_run("cost_estimation", estimated(Some(42)));
        for invalid in ["not json", "{}", "[]"] {
            fs::write(&path, invalid)?;
            assert!(update.merge_to_path(&path).is_err());
            assert_eq!(fs::read_to_string(&path)?, invalid);
        }
        let mut previous = serde_json::to_value(action_run(
            "execution",
            serde_json::json!({"crashed": {"reason": "old"}}),
        ))?;
        previous["execution"]["crashed"]["extra"] = serde_json::json!(42);
        previous["custom"] = serde_json::json!("keep");
        fs::write(&path, serde_json::to_vec(&previous)?)?;
        update.merge_to_path(&path)?;
        let merged: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)?;
        assert_eq!(merged["execution"], previous["execution"]);
        assert_eq!(merged["custom"], "keep");
        let mut wrong_name = update;
        wrong_name.name = "different".to_owned();
        let bytes = fs::read(&path)?;
        assert!(wrong_name.merge_to_path(&path).is_err());
        assert_eq!(fs::read(&path)?, bytes);
        Ok(())
    }
}
