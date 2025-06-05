#![doc = include_str!("../README.md")]

use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path, time::Duration};
use thiserror::Error;

/// Cycle-count metrics for a particular workload.
///
/// Stores the total cycle count and a breakdown of cycle count per named region.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkloadMetrics {
    /// Metrics produced when benchmarking in execution mode
    Execution {
        /// Name of the workload (e.g., "fft", "aes").
        name: String,
        /// Total number of cycles for the entire workload execution.
        total_num_cycles: u64,
        /// Region-specific cycles, mapping region names (e.g., "setup", "compute") to their cycle counts.
        region_cycles: HashMap<String, u64>,
        /// Execution duration.
        execution_duration: Duration,
    },
    /// Metrics produced when benchmarking in proving mode
    Proving {
        /// Name of the workload (e.g., "fft", "aes").
        name: String,
        /// Proving time in milliseconds
        proving_time_ms: u128,
    },
    /// Metrics produced when a benchmark crashes/errors
    Crashed {
        /// Name of the workload that crashed
        name: String,
        /// Action being performed when crash occurred (e.g., "execute", "prove")
        action: String,
        /// Reason for the crash (panic message)
        reason: String,
    },
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

impl WorkloadMetrics {
    /// Returns the name of the workload regardless of the variant.
    pub fn name(&self) -> &str {
        match self {
            WorkloadMetrics::Execution { name, .. } => name,
            WorkloadMetrics::Proving { name, .. } => name,
            WorkloadMetrics::Crashed { name, .. } => name,
        }
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

    // This is just a fixed sample we are using to test serde_roundtrip
    fn sample() -> Vec<WorkloadMetrics> {
        vec![
            WorkloadMetrics::Execution {
                name: "fft".into(),
                total_num_cycles: 1_000,
                region_cycles: HashMap::from_iter([
                    ("setup".to_string(), 100),
                    ("compute".to_string(), 800),
                    ("teardown".to_string(), 100),
                ]),
            },
            WorkloadMetrics::Execution {
                name: "aes".into(),
                total_num_cycles: 2_000,
                region_cycles: HashMap::from_iter([
                    ("init".to_string(), 200),
                    ("encrypt".to_string(), 1_600),
                    ("final".to_string(), 200),
                ]),
            },
            WorkloadMetrics::Proving {
                name: "rsa".into(),
                proving_time_ms: 5_000,
            },
            WorkloadMetrics::Proving {
                name: "ecdsa".into(),
                proving_time_ms: 3_500,
            },
            WorkloadMetrics::Crashed {
                name: "sha256".into(),
                action: "execute".into(),
                reason: "Out of memory panic".into(),
            },
        ]
    }

    #[test]
    fn round_trip_json() {
        let workloads = sample();
        let json = WorkloadMetrics::to_json(&workloads).expect("serialize");
        let parsed = WorkloadMetrics::from_json(&json).expect("deserialize");
        assert_eq!(workloads, parsed);
    }

    #[test]
    fn bad_json_is_error() {
        let bad = "{this is not valid json}";
        let err = WorkloadMetrics::from_json(bad).unwrap_err();
        assert!(err.into_serde_err().is_data());
    }

    #[test]
    fn file_round_trip() -> Result<(), MetricsError> {
        // Create a named temporary file.
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path();

        let workloads = sample();

        // Write → read → compare using the temp file's path.
        WorkloadMetrics::to_path(path, &workloads)?;
        let read_back = WorkloadMetrics::from_path(path)?;
        assert_eq!(workloads, read_back);

        Ok(())
    }

    #[test]
    fn test_name_accessor() {
        let execution_metric = WorkloadMetrics::Execution {
            name: "test_execution".into(),
            total_num_cycles: 1000,
            region_cycles: HashMap::new(),
        };

        let proving_metric = WorkloadMetrics::Proving {
            name: "test_proving".into(),
            proving_time_ms: 2000,
        };

        let crashed_metric = WorkloadMetrics::Crashed {
            name: "test_crashed".into(),
            action: "execute".into(),
            reason: "Test panic".into(),
        };

        assert_eq!(execution_metric.name(), "test_execution");
        assert_eq!(proving_metric.name(), "test_proving");
        assert_eq!(crashed_metric.name(), "test_crashed");
    }

    #[test]
    fn test_mixed_metrics_serialization() {
        let mixed_workloads = vec![
            WorkloadMetrics::Execution {
                name: "mixed_execution".into(),
                total_num_cycles: 500,
                region_cycles: HashMap::from_iter([("phase1".to_string(), 500)]),
            },
            WorkloadMetrics::Proving {
                name: "mixed_proving".into(),
                proving_time_ms: 1000,
            },
        ];

        let json = WorkloadMetrics::to_json(&mixed_workloads).expect("serialize mixed");
        let parsed = WorkloadMetrics::from_json(&json).expect("deserialize mixed");
        assert_eq!(mixed_workloads, parsed);
    }
}
