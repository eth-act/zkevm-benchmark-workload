#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub use chrono;

use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path, time::Duration};
use sysinfo::{CpuExt, System, SystemExt, Pid, ProcessExt};
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

/// Memory tracker for monitoring memory usage during proving.
#[derive(Debug, Clone)]
pub struct MemoryTracker {
    process_id: Pid,
    initial_memory: u64,
    peak_memory: u64,
    memory_samples: Vec<u64>,
}

impl MemoryTracker {
    /// Creates a new memory tracker
    pub fn new() -> Self {
        let process_id = Pid::from (std::process::id() as i32);

        Self {
            process_id,
            initial_memory: 0,
            peak_memory: 0,
            memory_samples: Vec::new(),
        }
    }

    /// Starts tracking memory usage
    pub fn start_tracking(&mut self) {
        let mut system = System::new_all();
        system.refresh_all();
        if let Some(process) = system.process(self.process_id) {
            self.initial_memory = process.memory() * 1024; // Convert from KiB to bytes
            self.peak_memory = self.initial_memory;
        }
    }

    /// Stops tracking memory usage
    pub fn stop_tracking(&mut self) {
        let mut system = System::new_all();
        system.refresh_all();
        if let Some(process) = system.process(self.process_id) {
            let final_memory = process.memory() * 1024; // Convert from KiB to bytes
            self.peak_memory = self.peak_memory.max(final_memory);
            self.memory_samples.push(final_memory);
        }
    }

    /// Samples the current memory usage and updates the peak memory usage if necessary.
    /// Also records the sampled memory usage for future analysis.
    pub fn sample_memory(&mut self) {
        let mut system = System::new_all();
        system.refresh_all();
        if let Some(process) = system.process(self.process_id) {
            let memory = process.memory() * 1024; // Convert from KiB to bytes
            self.peak_memory = self.peak_memory.max(memory);
            self.memory_samples.push(memory);
        }
    }

    /// Gets the average memory usage across all samples
    pub fn get_average_memory(&self) -> u64 {
        if self.memory_samples.is_empty() {
            return self.initial_memory;
        }
        self.memory_samples.iter().sum::<u64>() / self.memory_samples.len() as u64
    }

    /// Gets the peak memory usage in bytes
    pub fn get_peak_memory(&self) -> u64 {
        self.peak_memory
    }

    /// Gets the memory samples
    pub fn get_memory_samples(&self) -> &[u64] {
        &self.memory_samples
    }

    /// Gets the initial memory usage
    pub fn get_initial_memory(&self) -> u64 {
        self.initial_memory
    }
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
                /// Peak memory usage during proving in bytes.
                #[serde(skip_serializing_if = "Option::is_none")]
                peak_memory_usage_bytes: Option<u64>,
                /// Average memory usage during proving in bytes.
                #[serde(skip_serializing_if = "Option::is_none")]
                average_memory_usage_bytes: Option<u64>,
                /// Memory usage at start of proving in bytes.
                #[serde(skip_serializing_if = "Option::is_none")]
                initial_memory_usage_bytes: Option<u64>,
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
            Self::Serde(e) => e,
            Self::Io(e) => panic!("unexpected IO error in test: {e}"),
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
        ensure_parent_dirs(path)?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
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

fn ensure_parent_dirs<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
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
                    total_num_cycles: 1_000,
                    region_cycles: HashMap::from_iter([
                        ("setup".to_string(), 100),
                        ("compute".to_string(), 800),
                        ("teardown".to_string(), 100),
                    ]),
                    execution_duration: Duration::from_millis(150),
                }),
                proving: None,
            },
            BenchmarkRun {
                name: "aes_bench".into(),
                timestamp_completed: chrono::Utc::now(),
                metadata: Metadata {
                    block_gas_used: 67890,
                },
                execution: Some(ExecutionMetrics::Success {
                    total_num_cycles: 2_000,
                    region_cycles: HashMap::from_iter([
                        ("init".to_string(), 200),
                        ("encrypt".to_string(), 1_600),
                        ("final".to_string(), 200),
                    ]),
                    execution_duration: Duration::from_millis(300),
                }),
                proving: Some(ProvingMetrics::Success {
                    proof_size: 256,
                    proving_time_ms: 2_000,
                    peak_memory_usage_bytes: None,
                    average_memory_usage_bytes: None,
                    initial_memory_usage_bytes: None,
                }),
            },
            BenchmarkRun {
                name: "proving_bench".into(),
                timestamp_completed: chrono::Utc::now(),
                metadata: Metadata {
                    block_gas_used: 54321,
                },
                execution: None,
                proving: Some(ProvingMetrics::Success {
                    proof_size: 512,
                    proving_time_ms: 5_000,
                    peak_memory_usage_bytes: None,
                    average_memory_usage_bytes: None,
                    initial_memory_usage_bytes: None,
                }),
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
                total_num_cycles: 1000,
                region_cycles: HashMap::new(),
                execution_duration: Duration::from_millis(150),
            }),
            proving: None,
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
                total_num_cycles: 500,
                region_cycles: HashMap::from_iter([
                    ("setup".to_string(), 50),
                    ("compute".to_string(), 400),
                    ("teardown".to_string(), 50),
                ]),
                execution_duration: Duration::from_millis(100),
            }),
            proving: Some(ProvingMetrics::Success {
                proof_size: 128,
                proving_time_ms: 1500,
                peak_memory_usage_bytes: None,
                average_memory_usage_bytes: None,
                initial_memory_usage_bytes: None,
            }),
        };
        let json = BenchmarkRun::to_json(std::slice::from_ref(&bench)).expect("serialize mixed");
        let parsed = BenchmarkRun::from_json(&json).expect("deserialize mixed");
        assert_eq!(vec![bench], parsed);
    }
}
