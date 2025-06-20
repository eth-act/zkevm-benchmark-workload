#![doc = include_str!("../README.md")]

use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use thiserror::Error;

/// Represents a single benchmark run.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct BenchmarkRun {
    /// Name of the benchmark.
    pub name: String,
    /// Information about the hardware on which the benchmark was run.
    pub hardware: HardwareInfo,
    /// Metrics collected during run.
    pub actions_metrics: Vec<ActionMetrics>,
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

        HardwareInfo {
            cpu_model: system
                .cpus()
                .first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string()),
            total_ram_gib: system.total_memory() / (1024 * 1024 * 1024),
            gpus: detect_gpus(),
        }
    }
}

/// Detects available GPUs on the system.
fn detect_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();

    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=gpu_name")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
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
    }

    gpus
}

/// Information about a crash that occurred during a workload.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CrashInfo {
    /// The reason for the crash (e.g., panic message).
    pub reason: String,
}

/// Metrics for a particular action, either execution or proving.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ActionMetrics {
    /// Metrics produced when benchmarking in execution mode.
    Execution(ExecutionMetrics),
    /// Metrics produced when benchmarking in proving mode.
    Proving(ProvingMetrics),
}

/// Metrics for execution workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMetrics {
    /// Metrics for a successful execution workload.
    Success {
        /// Total number of cycles for the entire workload execution.
        total_num_cycles: u64,
        /// Region-specific cycles, mapping region names (e.g., "setup", "compute") to their cycle counts.
        region_cycles: HashMap<String, u64>,
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
        /// Proof size in bytes.
        proof_size: usize,
        /// Proving time in milliseconds.
        proving_time_ms: u128,
    },
    /// Metrics for a crashed proving workload.
    Crashed(CrashInfo),
}

/// Errors that can occur during metrics processing.
#[derive(Error, Debug)]
pub enum MetricsError {
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
            MetricsError::Serde(e) => e,
            MetricsError::Io(e) => panic!("unexpected IO error in test: {e}"),
        }
    }
}

impl BenchmarkRun {
    /// Returns the name of the benchmark.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Serializes a list of `WorkloadMetrics` into a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if serialization fails.
    pub fn to_json(items: &[Self]) -> Result<String, MetricsError> {
        serde_json::to_string(items).map_err(MetricsError::from)
    }

    /// Deserializes a list of `WorkloadMetrics` from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if deserialization fails.
    pub fn from_json(json: &str) -> Result<Vec<Self>, MetricsError> {
        serde_json::from_str(json).map_err(MetricsError::from)
    }

    /// Serializes `items` using JSON pretty-print and writes them to `path` atomically.
    ///
    /// The file is created if it does not exist and truncated if it does.
    /// Parent directories are created if they are missing.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if any filesystem operation fails.
    /// Returns `MetricsError::Serde` if JSON serialization fails.
    pub fn to_path<P: AsRef<Path>>(path: P, items: &[Self]) -> Result<(), MetricsError> {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            // `create_dir_all` is a no-op when the dirs are already there.
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(items)?;
        fs::write(path, json)?;

        Ok(())
    }

    /// Reads the file at `path` and deserializes a `Vec<WorkloadMetrics>` from its JSON content.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if reading the file fails.
    /// Returns `MetricsError::Serde` if JSON deserialization fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, MetricsError> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
    use tempfile::NamedTempFile;

    fn sample_hardware_info() -> HardwareInfo {
        HardwareInfo {
            cpu_model: "test_cpu".to_string(),
            total_ram_gib: 1,
            gpus: vec![],
        }
    }

    // This is just a fixed sample we are using to test serde_roundtrip
    fn sample() -> Vec<BenchmarkRun> {
        vec![
            BenchmarkRun {
                name: "fft_bench".into(),
                hardware: sample_hardware_info(),
                actions_metrics: vec![
                    ActionMetrics::Execution(ExecutionMetrics::Success {
                        total_num_cycles: 1_000,
                        region_cycles: HashMap::from_iter([
                            ("setup".to_string(), 100),
                            ("compute".to_string(), 800),
                            ("teardown".to_string(), 100),
                        ]),
                        execution_duration: Duration::from_millis(150),
                    }),
                    ActionMetrics::Execution(ExecutionMetrics::Crashed(CrashInfo {
                        reason: "panic in fft".into(),
                    })),
                ],
            },
            BenchmarkRun {
                name: "aes_bench".into(),
                hardware: sample_hardware_info(),
                actions_metrics: vec![ActionMetrics::Execution(ExecutionMetrics::Success {
                    total_num_cycles: 2_000,
                    region_cycles: HashMap::from_iter([
                        ("init".to_string(), 200),
                        ("encrypt".to_string(), 1_600),
                        ("final".to_string(), 200),
                    ]),
                    execution_duration: Duration::from_millis(300),
                })],
            },
            BenchmarkRun {
                name: "proving_bench".into(),
                hardware: sample_hardware_info(),
                actions_metrics: vec![
                    ActionMetrics::Proving(ProvingMetrics::Success {
                        proof_size: 512,
                        proving_time_ms: 5_000,
                    }),
                    ActionMetrics::Proving(ProvingMetrics::Crashed(CrashInfo {
                        reason: "proving failed".into(),
                    })),
                ],
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
        let err = BenchmarkRun::from_json(bad).unwrap_err();
        assert!(err.into_serde_err().is_data());
    }

    #[test]
    fn file_round_trip() -> Result<(), MetricsError> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();
        let runs = sample();
        BenchmarkRun::to_path(path, &runs)?;
        let read_back = BenchmarkRun::from_path(path)?;
        assert_eq!(runs, read_back);
        Ok(())
    }

    #[test]
    fn test_mixed_metrics_serialization() {
        let mixed_workloads = vec![
            ActionMetrics::Execution(ExecutionMetrics::Success {
                total_num_cycles: 500,
                region_cycles: HashMap::from_iter([("phase1".to_string(), 500)]),
                execution_duration: Duration::from_millis(200),
            }),
            ActionMetrics::Proving(ProvingMetrics::Success {
                proof_size: 300,
                proving_time_ms: 1000,
            }),
            ActionMetrics::Execution(ExecutionMetrics::Crashed(CrashInfo {
                reason: "fail".into(),
            })),
            ActionMetrics::Proving(ProvingMetrics::Crashed(CrashInfo {
                reason: "fail".into(),
            })),
        ];
        let bench = BenchmarkRun {
            name: "mixed_bench".into(),
            hardware: sample_hardware_info(),
            actions_metrics: mixed_workloads.clone(),
        };
        let json = BenchmarkRun::to_json(&[bench.clone()]).expect("serialize mixed");
        let parsed = BenchmarkRun::from_json(&json).expect("deserialize mixed");
        assert_eq!(vec![bench], parsed);
    }
}
